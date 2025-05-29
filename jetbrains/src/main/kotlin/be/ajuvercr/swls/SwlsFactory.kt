package be.ajuvercr.swls

import com.google.gson.JsonParser
import com.intellij.openapi.project.Project
import com.redhat.devtools.lsp4ij.LanguageServerFactory
import com.redhat.devtools.lsp4ij.client.LanguageClientImpl
import com.redhat.devtools.lsp4ij.server.ProcessStreamConnectionProvider
import com.redhat.devtools.lsp4ij.server.StreamConnectionProvider
import java.io.FileOutputStream
import java.net.HttpURLConnection
import java.net.URL
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.progress.ProgressManager


private val LOG = Logger.getInstance(SwlsServer::class.java)

class SwlsFactory : LanguageServerFactory {
    override fun createConnectionProvider(project: Project): StreamConnectionProvider = SwlsServer(project)
    override fun createLanguageClient(project: Project): LanguageClientImpl = SwlsLanguageClient(project)
}

class SwlsServer(project: Project) : ProcessStreamConnectionProvider() {
    init {
        var binaryPath: Path? = null

        ProgressManager.getInstance().runProcessWithProgressSynchronously(
            {
                try {
                    binaryPath = ensureLatestLspBinary()
                } catch (e: Exception) {
                    LOG.error("Failed to prepare LSP binary", e)
                }
            },
            "Preparing Semantic Web Language Server...",
            false, // don't allow cancellation
            null   // project context (can be null)
        )

        if (binaryPath == null) {
            throw RuntimeException("Could not prepare Semantic Web Language Server binary.")
        }

        // tell LSP4IntelliJ to launch the downloaded binary
        super.setCommands(listOf(binaryPath!!.toAbsolutePath().toString()))
    }

    private fun ensureLatestLspBinary(): Path {
        // 1. Prepare cache directories
        val cacheDir = Paths.get(System.getProperty("user.home"), ".cache", "swls", "bin")
        Files.createDirectories(cacheDir)

        // 2. Fetch latest release metadata from GitHub
        val apiUrl = "https://api.github.com/repos/ajuvercr/semantic-web-lsp/releases/latest"
        val conn = (URL(apiUrl).openConnection() as HttpURLConnection).apply {
            requestMethod = "GET"
            setRequestProperty("Accept", "application/vnd.github.v3+json")
        }

        if (conn.responseCode != 200) {
            throw RuntimeException("Failed to fetch latest release: HTTP ${conn.responseCode}")
        }

        val payload = conn.inputStream.bufferedReader().readText()
        val json  = JsonParser.parseString(payload).asJsonObject
        val latestTag = json.get("tag_name").asString

        // 3. Determine the correct asset name for this OS
        val os   = System.getProperty("os.name").lowercase()
        val assetName = when {
            os.contains("win")  -> "lsp_bin-windows-x86_64.exe"
            os.contains("mac")  -> "lsp_bin-macos-x86_64"
            else                -> "lsp_bin-linux-x86_64"
        }

        // 4. Paths to the binary and version marker
        val binPath     = cacheDir.resolve(assetName)
        val versionFile = cacheDir.resolve("version.txt")

        // 5. Compare versions
        val needDownload = when {
            !Files.exists(binPath)               -> true
            !Files.exists(versionFile)           -> true
            Files.readString(versionFile) != latestTag -> true
            else                                 -> false
        }

        if (needDownload) {
            // find the right asset URL
            val assets = json.getAsJsonArray("assets")
            val assetUrl = assets
                .first { it.asJsonObject.get("name").asString == assetName }
                .asJsonObject
                .get("browser_download_url")
                .asString

            // download it
            URL(assetUrl).openStream().use { input ->
                FileOutputStream(binPath.toFile()).use { output ->
                    input.copyTo(output)
                }
            }
            if (!os.contains("win")) {
                binPath.toFile().setExecutable(true, false)
            }
            // record the downloaded version
            Files.writeString(versionFile, latestTag)
        }

        LOG.info("Using SWLS binary at: $binPath (version: $latestTag)")

        return binPath
    }
}

// enabling the semanticTokens plugin is required for semantic tokens support
class SwlsLanguageClient(project: Project) : LanguageClientImpl(project) {
}