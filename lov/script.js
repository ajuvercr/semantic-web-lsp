const { readFile, writeFile } = require("fs/promises");

async function getInfo(prefix) {
  const url =
    "https://lov.linkeddata.es/dataset/lov/api/v2/vocabulary/info?vocab=" +
    prefix;
  const resp = await fetch(url);
  const json = await resp.json();
  return json;
}

function getLatestVersion(json) {
  let out = null;
  let out_date = null;
  for (const v of json.versions) {
    if (!v.issued || !v.fileURL) continue;
    try {
      const date = new Date(v.issued);
      if (!out_date || date > out_date) {
        out = v.fileURL;
        out_date = date;
      }
    } catch (ex) {
    }
  }

  return out;
}

async function fetchLov(prefix) {
  const info = await getInfo(prefix);
  const url = getLatestVersion(info);

  const resp = await fetch(url);
  return await resp.text();
}

const SOURCE_HEADER = `
pub struct LocalPrefix {
    pub location: &'static str,
    pub content: &'static str,
    pub name: &'static str,
    pub title: &'static str,
}

pub const LOCAL_PREFIXES: &'static [LocalPrefix] = 
`;
async function main() {
  const listRequest = await fetch("https://lov.linkeddata.es/dataset/lov/api/v2/vocabulary/list");
  const inp = await listRequest.json();
  const out = [];
  for (const thing of inp) {
    console.log("handling", thing.prefix);
    try {
      const title = thing.titles.find((x) => x.lang === "en")?.value ||
        thing.prefix;

      const text = await fetchLov(thing.prefix);
      await writeFile(`./prefixes/${thing.prefix}.ttl`, text, {
        "encoding": "utf8",
      });

      out.push(`LocalPrefix {
  location: "${thing.uri}",
  content: include_str!("../prefixes/${thing.prefix}.ttl"),
  name: "${thing.prefix}",
  title: "${title}",
}`);
    } catch (ex) {
      console.log("Failed", thing.prefix, ex);
    }
  }

  await writeFile("main.rs", `${SOURCE_HEADER}&[${out.join(", ")}]`, { encoding: "utf8" });
}
main();
