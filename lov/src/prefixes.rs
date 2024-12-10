
use super::LocalPrefix;

pub const LOCAL_PREFIXES: &'static [LocalPrefix] = 
&[LocalPrefix {
  location: "https://w3id.org/CPSWatch",
  content: include_str!("../prefixes/CPSWatch.ttl"),
  name: "CPSWatch",
  title: "The Cyber Physical System Watch Ontology",
}, LocalPrefix {
  location: "https://w3id.org/IIoT",
  content: include_str!("../prefixes/IIoT.ttl"),
  name: "IIoT",
  title: "Ontology for Industrial Internet of Things systems",
}, LocalPrefix {
  location: "http://www.irit.fr/recherches/MELODI/ontologies/SAN",
  content: include_str!("../prefixes/SAN.ttl"),
  name: "SAN",
  title: "SAN (Semantic Actuator Network)",
}, LocalPrefix {
  location: "https://w3id.org/arco/ontology/location",
  content: include_str!("../prefixes/a-loc.ttl"),
  name: "a-loc",
  title: "Location Ontology (ArCo network)",
}, LocalPrefix {
  location: "http://rs.tdwg.org/ac/terms/",
  content: include_str!("../prefixes/ac.ttl"),
  name: "ac",
  title: "Core terms defined by Audubon Core",
}, LocalPrefix {
  location: "http://purl.org/acco/ns",
  content: include_str!("../prefixes/acco.ttl"),
  name: "acco",
  title: "Accomodation Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/ns/auth/acl",
  content: include_str!("../prefixes/acl.ttl"),
  name: "acl",
  title: "Basic Access Control ontology",
}, LocalPrefix {
  location: "http://www.rkbexplorer.com/ontologies/acm",
  content: include_str!("../prefixes/acm.ttl"),
  name: "acm",
  title: "ACM Classification Ontology",
}, LocalPrefix {
  location: "http://privatealpha.com/ontology/certification/1#",
  content: include_str!("../prefixes/acrt.ttl"),
  name: "acrt",
  title: "Agent Certification Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/ns/adms",
  content: include_str!("../prefixes/adms.ttl"),
  name: "adms",
  title: "Asset Description Metadata Schema",
}, LocalPrefix {
  location: "http://purl.org/ontology/af/",
  content: include_str!("../prefixes/af.ttl"),
  name: "af",
  title: "Audio Features Ontology",
}, LocalPrefix {
  location: "http://www.agls.gov.au/agls/terms",
  content: include_str!("../prefixes/agls.ttl"),
  name: "agls",
  title: "AGLS Metadata Terms",
}, LocalPrefix {
  location: "http://promsns.org/def/agr",
  content: include_str!("../prefixes/agr.ttl"),
  name: "agr",
  title: "Agreements ontology",
}, LocalPrefix {
  location: "http://d-nb.info/standards/elementset/agrelon",
  content: include_str!("../prefixes/agrelon.ttl"),
  name: "agrelon",
  title: "Agent Relationship Ontology",
}, LocalPrefix {
  location: "http://purl.org/vocab/aiiso/schema",
  content: include_str!("../prefixes/aiiso.ttl"),
  name: "aiiso",
  title: "Academic Institution Internal Structure Ontology",
}, LocalPrefix {
  location: "https://w3id.org/airo",
  content: include_str!("../prefixes/airo.ttl"),
  name: "airo",
  title: "AI Risk Ontology",
}, LocalPrefix {
  location: "https://raw.githubusercontent.com/airs-linked-data/lov/latest/src/airs_vocabulary.ttl#",
  content: include_str!("../prefixes/airs.ttl"),
  name: "airs",
  title: "Alliance of Information and Referral Services (AIRS) Vocabulary",
}, LocalPrefix {
  location: "http://www.aktors.org/ontology/portal",
  content: include_str!("../prefixes/akt.ttl"),
  name: "akt",
  title: "AKT Reference Ontology",
}, LocalPrefix {
  location: "http://www.aktors.org/ontology/support",
  content: include_str!("../prefixes/akts.ttl"),
  name: "akts",
  title: "AKT Support Ontology",
}, LocalPrefix {
  location: "http://securitytoolbox.appspot.com/securityAlgorithms#",
  content: include_str!("../prefixes/algo.ttl"),
  name: "algo",
  title: "Algorithms Ontology",
}, LocalPrefix {
  location: "http://open-services.net/ns/asset#",
  content: include_str!("../prefixes/am.ttl"),
  name: "am",
  title: "OSLC Asset Management Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/i40/aml",
  content: include_str!("../prefixes/aml.ttl"),
  name: "aml",
  title: "AutomationML Ontology",
}, LocalPrefix {
  location: "http://w3id.org/amlo/core",
  content: include_str!("../prefixes/amlo-core.ttl"),
  name: "amlo-core",
  title: "AMLO-core vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/amv",
  content: include_str!("../prefixes/amv.ttl"),
  name: "amv",
  title: "AMV:Algorithm Metadata Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/ontology/ao/core#",
  content: include_str!("../prefixes/ao.ttl"),
  name: "ao",
  title: "Association Ontology",
}, LocalPrefix {
  location: "http://rdf.muninn-project.org/ontologies/appearances",
  content: include_str!("../prefixes/aos.ttl"),
  name: "aos",
  title: "Appearances Ontology Specification",
}, LocalPrefix {
  location: "https://purl.org/cm/onto/apco",
  content: include_str!("../prefixes/apco.ttl"),
  name: "apco",
  title: "African Public Contract Ontology",
}, LocalPrefix {
  location: "http://purl.org/linked-data/api/vocab#",
  content: include_str!("../prefixes/api.ttl"),
  name: "api",
  title: "Linked Data API Vocabulary",
}, LocalPrefix {
  location: "http://semweb.mmlab.be/ns/apps4X",
  content: include_str!("../prefixes/apps4X.ttl"),
  name: "apps4X",
  title: "The vocabulary for Co-creation Events based on Open Data",
}, LocalPrefix {
  location: "http://purl.org/archival/vocab/arch",
  content: include_str!("../prefixes/arch.ttl"),
  name: "arch",
  title: "Archival collections ontology",
}, LocalPrefix {
  location: "https://w3id.org/arco/ontology/core",
  content: include_str!("../prefixes/arco.ttl"),
  name: "arco",
  title: "Core Ontology (ArCo network)",
}, LocalPrefix {
  location: "http://www.arpenteur.org/ontology/Arpenteur.owl",
  content: include_str!("../prefixes/arp.ttl"),
  name: "arp",
  title: "Arpenteur Ontology",
}, LocalPrefix {
  location: "https://data.nasa.gov/ontologies/atmonto/data#",
  content: include_str!("../prefixes/atd.ttl"),
  name: "atd",
  title: "Air Traffic Data",
}, LocalPrefix {
  location: "https://data.nasa.gov/ontologies/atmonto/ATM#",
  content: include_str!("../prefixes/atm.ttl"),
  name: "atm",
  title: "Air Traffic Management (ATM) Vocabulary",
}, LocalPrefix {
  location: "https://data.nasa.gov/ontologies/atmonto/general#",
  content: include_str!("../prefixes/atts.ttl"),
  name: "atts",
  title: "Air Traffic Temporal and Spacial Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/inrupt/namespace/vocab/authn_provider/",
  content: include_str!("../prefixes/authn_provider.ttl"),
  name: "authn_provider",
  title: "Authentication Provider",
}, LocalPrefix {
  location: "http://bblfish.net/work/atom-owl/2006-06-06/",
  content: include_str!("../prefixes/awol.ttl"),
  name: "awol",
  title: "Atom Syndication Ontology",
}, LocalPrefix {
  location: "http://purl.oclc.org/NET/ssnx/meteo/aws",
  content: include_str!("../prefixes/aws.ttl"),
  name: "aws",
  title: "Ontology for Meteorological sensors",
}, LocalPrefix {
  location: "http://bag.basisregistraties.overheid.nl/def/bag",
  content: include_str!("../prefixes/bag.ttl"),
  name: "bag",
  title: "Vocabulary for the Dutch base registration of buildings and addresses (BAG)",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19103/2005/basic",
  content: include_str!("../prefixes/basic.ttl"),
  name: "basic",
  title: "OWL representation of ISO 19103 (Basic types package)",
}, LocalPrefix {
  location: "http://w3id.org/emmo-bto/bto",
  content: include_str!("../prefixes/bato.ttl"),
  name: "bato",
  title: "Battery Testing Ontology",
}, LocalPrefix {
  location: "http://www.bbc.co.uk/ontologies/bbc",
  content: include_str!("../prefixes/bbc.ttl"),
  name: "bbc",
  title: "BBC Ontology",
}, LocalPrefix {
  location: "http://www.bbc.co.uk/ontologies/cms",
  content: include_str!("../prefixes/bbccms.ttl"),
  name: "bbccms",
  title: "BBC CMS Ontology",
}, LocalPrefix {
  location: "http://www.bbc.co.uk/ontologies/coreconcepts",
  content: include_str!("../prefixes/bbccore.ttl"),
  name: "bbccore",
  title: "BBC Core Concepts",
}, LocalPrefix {
  location: "http://www.bbc.co.uk/ontologies/provenance",
  content: include_str!("../prefixes/bbcprov.ttl"),
  name: "bbcprov",
  title: "BBC Provenance Ontology",
}, LocalPrefix {
  location: "https://w3id.org/BCI-ontology",
  content: include_str!("../prefixes/bci.ttl"),
  name: "bci",
  title: "Brain Computing Interface (BCI) Ontology",
}, LocalPrefix {
  location: "https://w3id.org/bcom",
  content: include_str!("../prefixes/bcom.ttl"),
  name: "bcom",
  title: "Building Concrete Monitoring Ontology (BCOM)",
}, LocalPrefix {
  location: "http://contextus.net/ontology/ontomedia/ext/common/being#",
  content: include_str!("../prefixes/being.ttl"),
  name: "being",
  title: "OntoMedia Being Representation",
}, LocalPrefix {
  location: "http://rdfs.co/bevon/",
  content: include_str!("../prefixes/bevon.ttl"),
  name: "bevon",
  title: "BEVON: Beverage Ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/bibo/",
  content: include_str!("../prefixes/bibo.ttl"),
  name: "bibo",
  title: "The Bibliographic Ontology",
}, LocalPrefix {
  location: "http://purl.org/net/nknouf/ns/bibtex",
  content: include_str!("../prefixes/bibtex.ttl"),
  name: "bibtex",
  title: "BibTeX ontology",
}, LocalPrefix {
  location: "http://bimerr.iot.linkeddata.es/def/occupancy-profile#",
  content: include_str!("../prefixes/bimerr-op.ttl"),
  name: "bimerr-op",
  title: "Occupancy Profile ontology",
}, LocalPrefix {
  location: "http://purl.org/vocab/bio/0.1/",
  content: include_str!("../prefixes/bio.ttl"),
  name: "bio",
  title: "BIO: A vocabulary for biographical information",
}, LocalPrefix {
  location: "http://purl.org/NET/biol/ns#",
  content: include_str!("../prefixes/biol.ttl"),
  name: "biol",
  title: "Biological Taxonomy Vocabulary 0.2 (Core)",
}, LocalPrefix {
  location: "http://www.biopax.org/release/biopax-level3.owl",
  content: include_str!("../prefixes/biopax.ttl"),
  name: "biopax",
  title: "BioPAX Level 3 ontology",
}, LocalPrefix {
  location: "http://purl.org/biotop/biotop.owl",
  content: include_str!("../prefixes/biotop.ttl"),
  name: "biotop",
  title: "BioTop",
}, LocalPrefix {
  location: "http://purl.org/spar/biro",
  content: include_str!("../prefixes/biro.ttl"),
  name: "biro",
  title: "The Bibliographic Reference Ontology",
}, LocalPrefix {
  location: "http://www.bl.uk/schemas/bibliographic/blterms",
  content: include_str!("../prefixes/blt.ttl"),
  name: "blt",
  title: "British Library Terms RDF schema",
}, LocalPrefix {
  location: "http://swa.cefriel.it/ontologies/botdcat-ap",
  content: include_str!("../prefixes/bot.ttl"),
  name: "bot",
  title: "BotDCAT-AP - Data Catalogue vocabulary Application Profile for chatbots",
}, LocalPrefix {
  location: "http://purl.org/NET/biol/botany#",
  content: include_str!("../prefixes/botany.ttl"),
  name: "botany",
  title: "Biological Taxonomy Vocabulary 0.2 (Botany)",
}, LocalPrefix {
  location: "http://data.vlaanderen.be/ns/persoon",
  content: include_str!("../prefixes/bperson.ttl"),
  name: "bperson",
  title: "Person",
}, LocalPrefix {
  location: "http://vocab.deri.ie/br",
  content: include_str!("../prefixes/br.ttl"),
  name: "br",
  title: "Brainstorm Ontology",
}, LocalPrefix {
  location: "http://brk.basisregistraties.overheid.nl/def/brk",
  content: include_str!("../prefixes/brk.ttl"),
  name: "brk",
  title: "Key Register Cadastre (BRK) vocabulary",
}, LocalPrefix {
  location: "http://brt.basisregistraties.overheid.nl/def/top10nl",
  content: include_str!("../prefixes/brt.ttl"),
  name: "brt",
  title: "Key Register Topography (BRT) vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/bot",
  content: include_str!("../prefixes/bto.ttl"),
  name: "bto",
  title: "BOT: Building Topology Ontology",
}, LocalPrefix {
  location: "http://vocab.deri.ie/c4n",
  content: include_str!("../prefixes/c4n.ttl"),
  name: "c4n",
  title: "Call for Anything vocabulary",
}, LocalPrefix {
  location: "http://purl.org/spar/c4o",
  content: include_str!("../prefixes/c4o.ttl"),
  name: "c4o",
  title: "C4O, the Citation Counting and Context Characterization Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2002/12/cal/ical",
  content: include_str!("../prefixes/cal.ttl"),
  name: "cal",
  title: "Internet Calendaring and Scheduling Core Object Specification",
}, LocalPrefix {
  location: "http://caressesrobot.org/ontology",
  content: include_str!("../prefixes/caresses.ttl"),
  name: "caresses",
  title: "CARESSES Ontology",
}, LocalPrefix {
  location: "http://purl.org/net/cartCoord#",
  content: include_str!("../prefixes/cart.ttl"),
  name: "cart",
  title: "cartCoordOnt",
}, LocalPrefix {
  location: "http://www.w3id.org/def/caso#",
  content: include_str!("../prefixes/caso.ttl"),
  name: "caso",
  title: "Context Aware System Observation Ontology",
}, LocalPrefix {
  location: "http://w3id.org/um/cbcm/eu-cm-ontology",
  content: include_str!("../prefixes/cbcm.ttl"),
  name: "cbcm",
  title: "The European Union Company Mobility Ontology (EUCM ontology)",
}, LocalPrefix {
  location: "http://comicmeta.org/cbo/",
  content: include_str!("../prefixes/cbo.ttl"),
  name: "cbo",
  title: "Comic Book Ontology",
}, LocalPrefix {
  location: "http://betalinkeddata.cbs.nl/def/cbs",
  content: include_str!("../prefixes/cbs.ttl"),
  name: "cbs",
  title: "Netherlands' National Statistics Office (CBS) ontology",
}, LocalPrefix {
  location: "http://creativecommons.org/ns",
  content: include_str!("../prefixes/cc.ttl"),
  name: "cc",
  title: "Creative Commons Rights Expression Language",
}, LocalPrefix {
  location: "http://cookingbigdata.com/linkeddata/ccinstances",
  content: include_str!("../prefixes/cci.ttl"),
  name: "cci",
  title: "Ontology for Cloud Computing instances",
}, LocalPrefix {
  location: "http://purl.org/ontology/cco/core#",
  content: include_str!("../prefixes/cco.ttl"),
  name: "cco",
  title: "Cognitive Characteristics Ontology",
}, LocalPrefix {
  location: "http://cookingbigdata.com/linkeddata/ccpricing",
  content: include_str!("../prefixes/ccp.ttl"),
  name: "ccp",
  title: "Vocabulary for prices options in Cloud Computing Services",
}, LocalPrefix {
  location: "http://cookingbigdata.com/linkeddata/ccregions",
  content: include_str!("../prefixes/ccr.ttl"),
  name: "ccr",
  title: "Vocabulary for Regions and Zones on Cloud Computing",
}, LocalPrefix {
  location: "http://cookingbigdata.com/linkeddata/ccsla",
  content: include_str!("../prefixes/ccsla.ttl"),
  name: "ccsla",
  title: "Service Level Agreement for Cloud Computing",
}, LocalPrefix {
  location: "https://w3id.org/cdc",
  content: include_str!("../prefixes/cdc.ttl"),
  name: "cdc",
  title: "CDC: Construction Dataset Context ontology",
}, LocalPrefix {
  location: "https://w3id.org/arco/ontology/context-description",
  content: include_str!("../prefixes/cdesc.ttl"),
  name: "cdesc",
  title: "Context Description Ontology (ArCo network)",
}, LocalPrefix {
  location: "http://purl.org/twc/ontology/cdm.owl#",
  content: include_str!("../prefixes/cdm.ttl"),
  name: "cdm",
  title: "Conceptual Depth and Momentum",
}, LocalPrefix {
  location: "http://purl.org/cld/cdtype/",
  content: include_str!("../prefixes/cdtype.ttl"),
  name: "cdtype",
  title: "The Collection Description Type Namespace",
}, LocalPrefix {
  location: "https://w3id.org/CEMontology",
  content: include_str!("../prefixes/cem.ttl"),
  name: "cem",
  title: "Crime Event Model (CEM)",
}, LocalPrefix {
  location: "http://www.ebusiness-unibw.org/ontologies/consumerelectronics/v1",
  content: include_str!("../prefixes/ceo.ttl"),
  name: "ceo",
  title: "Consumer Electronics Ontology",
}, LocalPrefix {
  location: "http://www.eurocris.org/ontologies/cerif/1.3",
  content: include_str!("../prefixes/cerif.ttl"),
  name: "cerif",
  title: "CERIF Ontology 1.3",
}, LocalPrefix {
  location: "http://www.w3.org/ns/auth/cert#",
  content: include_str!("../prefixes/cert.ttl"),
  name: "cert",
  title: "The Cert Ontology",
}, LocalPrefix {
  location: "https://w3id.org/arco/ontology/cultural-event",
  content: include_str!("../prefixes/cevent.ttl"),
  name: "cevent",
  title: "Cultural Event Ontology (ArCo network)",
}, LocalPrefix {
  location: "http://purl.oclc.org/NET/ssnx/cf/cf-feature",
  content: include_str!("../prefixes/cff.ttl"),
  name: "cff",
  title: "Climate and Forecast (CF) features",
}, LocalPrefix {
  location: "http://purl.oclc.org/NET/ssnx/cf/cf-property",
  content: include_str!("../prefixes/cfp.ttl"),
  name: "cfp",
  title: "Climate and Forecast (CF) standard names parameter vocabulary",
}, LocalPrefix {
  location: "http://linkeddata.finki.ukim.mk/lod/ontology/cfrl#",
  content: include_str!("../prefixes/cfrl.ttl"),
  name: "cfrl",
  title: "Corporate Financial Reports and Loans Ontology",
}, LocalPrefix {
  location: "http://reference.data.gov.uk/def/central-government",
  content: include_str!("../prefixes/cgov.ttl"),
  name: "cgov",
  title: "Central Government Ontology",
}, LocalPrefix {
  location: "https://w3id.org/emmo/domain/characterisation-methodology/chameo",
  content: include_str!("../prefixes/chameo.ttl"),
  name: "chameo",
  title: "CHAracterisation MEthodology Ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/chord/",
  content: include_str!("../prefixes/chord.ttl"),
  name: "chord",
  title: "The OMRAS2 Chord Ontology",
}, LocalPrefix {
  location: "https://privatealpha.com/ontology/content-inventory/1#",
  content: include_str!("../prefixes/ci.ttl"),
  name: "ci",
  title: "A Content Inventory Vocabulary",
}, LocalPrefix {
  location: "http://dati.beniculturali.it/cultural-ON/cultural-ON.owl",
  content: include_str!("../prefixes/cis.ttl"),
  name: "cis",
  title: "Cultural-ON (Cultural ONtology): Cultural Institute/Site and Cultural Event Ontology",
}, LocalPrefix {
  location: "https://w3id.org/citedcat-ap",
  content: include_str!("../prefixes/citedcat.ttl"),
  name: "citedcat",
  title: "CiteDCAT-AP Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/spar/cito",
  content: include_str!("../prefixes/cito.ttl"),
  name: "cito",
  title: "CiTO, the Citation Typing Ontology",
}, LocalPrefix {
  location: "http://www.essepuntato.it/2013/03/cito-functions",
  content: include_str!("../prefixes/citof.ttl"),
  name: "citof",
  title: "Functions of Citations",
}, LocalPrefix {
  location: "http://advene.org/ns/cinelab/ld",
  content: include_str!("../prefixes/cl.ttl"),
  name: "cl",
  title: "Cinelab ontology",
}, LocalPrefix {
  location: "http://purl.org/cld/terms/",
  content: include_str!("../prefixes/cld.ttl"),
  name: "cld",
  title: "The Collection Description Terms",
}, LocalPrefix {
  location: "https://w3id.org/cmd#",
  content: include_str!("../prefixes/cmd.ttl"),
  name: "cmd",
  title: "Compound Measure Description",
}, LocalPrefix {
  location: "http://purl.org/twc/ontologies/cmo.owl",
  content: include_str!("../prefixes/cmo.ttl"),
  name: "cmo",
  title: "Conceptual Model Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2011/content",
  content: include_str!("../prefixes/cnt.ttl"),
  name: "cnt",
  title: "Representing Content in RDF",
}, LocalPrefix {
  location: "http://purl.org/ontology/co/core#",
  content: include_str!("../prefixes/co.ttl"),
  name: "co",
  title: "Counter Ontology",
}, LocalPrefix {
  location: "http://data.cochrane.org/ontologies/core/",
  content: include_str!("../prefixes/cochrane.ttl"),
  name: "cochrane",
  title: "Cochrane Core Vocabulary Ontology",
}, LocalPrefix {
  location: "https://w3id.org/cocoon/v1.0",
  content: include_str!("../prefixes/cocoon.ttl"),
  name: "cocoon",
  title: "Cloud Computing Services Ontology",
}, LocalPrefix {
  location: "http://vocab.deri.ie/cogs",
  content: include_str!("../prefixes/cogs.ttl"),
  name: "cogs",
  title: "COGS Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/configurationontology",
  content: include_str!("../prefixes/cold.ttl"),
  name: "cold",
  title: "'Configuration as Linked Data' ontology",
}, LocalPrefix {
  location: "http://purl.org/co",
  content: include_str!("../prefixes/coll.ttl"),
  name: "coll",
  title: "Collections Ontology",
}, LocalPrefix {
  location: "http://vocab.resc.info/communication",
  content: include_str!("../prefixes/comm.ttl"),
  name: "comm",
  title: "Vocabulary related to incident communication",
}, LocalPrefix {
  location: "http://www.w3.org/2007/uwa/context/deliverycontext.owl",
  content: include_str!("../prefixes/common.ttl"),
  name: "common",
  title: "The Delivery Context Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2000/10/swap/pim/contact",
  content: include_str!("../prefixes/con.ttl"),
  name: "con",
  title: "Contact",
}, LocalPrefix {
  location: "https://w3id.org/con-tax",
  content: include_str!("../prefixes/contax.ttl"),
  name: "contax",
  title: "ConTax ontology",
}, LocalPrefix {
  location: "http://purl.org/twc/vocab/conversion/",
  content: include_str!("../prefixes/conversion.ttl"),
  name: "conversion",
  title: "Conversion Ontology",
}, LocalPrefix {
  location: "http://purl.org/coo/ns#",
  content: include_str!("../prefixes/coo.ttl"),
  name: "coo",
  title: "Car Options Ontology",
}, LocalPrefix {
  location: "https://w3id.org/mdo/core/",
  content: include_str!("../prefixes/core.ttl"),
  name: "core",
  title: "Materials Design Ontology - Core Module",
}, LocalPrefix {
  location: "http://purl.org/coreo",
  content: include_str!("../prefixes/coreo.ttl"),
  name: "coreo",
  title: "Core-o: Competence Reference Ontology",
}, LocalPrefix {
  location: "http://www.daml.org/2001/09/countries/iso-3166-ont",
  content: include_str!("../prefixes/coun.ttl"),
  name: "coun",
  title: "ISO 3166 Country Codes",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/schemas/cpannotationschema.owl",
  content: include_str!("../prefixes/cpa.ttl"),
  name: "cpa",
  title: "Content Pattern Annotations",
}, LocalPrefix {
  location: "http://meta.icos-cp.eu/ontologies/cpmeta/",
  content: include_str!("../prefixes/cpmeta.ttl"),
  name: "cpmeta",
  title: "Ontology of Integrated Carbon Observation System (ICOS)",
}, LocalPrefix {
  location: "http://www.cidoc-crm.org/cidoc-crm/",
  content: include_str!("../prefixes/crm.ttl"),
  name: "crm",
  title: "CIDOC Conceptual Reference Model",
}, LocalPrefix {
  location: "http://rhizomik.net/ontologies/copyrightonto.owl",
  content: include_str!("../prefixes/cro.ttl"),
  name: "cro",
  title: "Copyright Ontology",
}, LocalPrefix {
  location: "http://courseware.rkbexplorer.com/ontologies/courseware",
  content: include_str!("../prefixes/crsw.ttl"),
  name: "crsw",
  title: "ReSIST Courseware Ontology",
}, LocalPrefix {
  location: "http://purl.org/vocab/changeset/schema",
  content: include_str!("../prefixes/cs.ttl"),
  name: "cs",
  title: "Changeset",
}, LocalPrefix {
  location: "http://vocab.deri.ie/csp",
  content: include_str!("../prefixes/csp.ttl"),
  name: "csp",
  title: "Constraint Satisfaction Problems Vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/csvw#",
  content: include_str!("../prefixes/csvw.ttl"),
  name: "csvw",
  title: "CSV on the Web Vocabulary",
}, LocalPrefix {
  location: "http://www.tele.pw.edu.pl/~sims-onto/ConnectivityType.owl",
  content: include_str!("../prefixes/ct.ttl"),
  name: "ct",
  title: "Connectivity types",
}, LocalPrefix {
  location: "http://commontag.org/ns#",
  content: include_str!("../prefixes/ctag.ttl"),
  name: "ctag",
  title: "Common Tag Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/arco/ontology/catalogue",
  content: include_str!("../prefixes/ctlog.ttl"),
  name: "ctlog",
  title: "Catalogue Ontology (ArCo network)",
}, LocalPrefix {
  location: "https://w3id.org/cto",
  content: include_str!("../prefixes/cto.ttl"),
  name: "cto",
  title: "CTO: Construction Tasks Ontology",
}, LocalPrefix {
  location: "http://purl.org/ctic/infraestructuras/organizacion",
  content: include_str!("../prefixes/ctorg.ttl"),
  name: "ctorg",
  title: "Vocabulary for the structure of the public organizations",
}, LocalPrefix {
  location: "https://w3id.org/ibp/CTRLont",
  content: include_str!("../prefixes/ctrl.ttl"),
  name: "ctrl",
  title: "CTRLont - An ontology to formally specify the control domain",
}, LocalPrefix {
  location: "http://www.demcare.eu/ontologies/contextdescriptor.owl",
  content: include_str!("../prefixes/ctxdesc.ttl"),
  name: "ctxdesc",
  title: "Context Descriptor Pattern",
}, LocalPrefix {
  location: "http://purl.org/cwmo/#",
  content: include_str!("../prefixes/cwmo.ttl"),
  name: "cwmo",
  title: "Creative Workshop Management Ontology (CWMO)",
}, LocalPrefix {
  location: "http://www.bbc.co.uk/ontologies/creativework",
  content: include_str!("../prefixes/cwork.ttl"),
  name: "cwork",
  title: "Creative Work Ontology",
}, LocalPrefix {
  location: "http://sparql.cwrc.ca/ontologies/cwrc",
  content: include_str!("../prefixes/cwrc.ttl"),
  name: "cwrc",
  title: "The CWRC Ontology",
}, LocalPrefix {
  location: "http://www.wiwiss.fu-berlin.de/suhl/bizer/D2RQ/0.1",
  content: include_str!("../prefixes/d2rq.ttl"),
  name: "d2rq",
  title: "D2RQ - Language Specification",
}, LocalPrefix {
  location: "http://vocab.deri.ie/dady",
  content: include_str!("../prefixes/dady.ttl"),
  name: "dady",
  title: "Dataset Dynamics (dady) vocabulary",
}, LocalPrefix {
  location: "http://purl.org/ontology/daia",
  content: include_str!("../prefixes/daia.ttl"),
  name: "daia",
  title: "Document Availability Information Ontology",
}, LocalPrefix {
  location: "http://purl.org/eis/vocab/daq#",
  content: include_str!("../prefixes/daq.ttl"),
  name: "daq",
  title: "Dataset Quality Vocabulary",
}, LocalPrefix {
  location: "http://dataid.dbpedia.org/ns/core#",
  content: include_str!("../prefixes/dataid.ttl"),
  name: "dataid",
  title: "DataID",
}, LocalPrefix {
  location: "http://contextus.net/ontology/ontomedia/misc/date#",
  content: include_str!("../prefixes/date.ttl"),
  name: "date",
  title: "OntoMedia Date Part Representation",
}, LocalPrefix {
  location: "http://vocab.datex.org/terms#",
  content: include_str!("../prefixes/datex.ttl"),
  name: "datex",
  title: "Linked Datex II",
}, LocalPrefix {
  location: "http://theme-e.adaptcentre.ie/dave/dave.ttl",
  content: include_str!("../prefixes/dave.ttl"),
  name: "dave",
  title: "Data Value Vocabulary (DaVe)",
}, LocalPrefix {
  location: "https://w3id.org/dba/ontology/",
  content: include_str!("../prefixes/dba.ttl"),
  name: "dba",
  title: "Description Banking Archives Ontology",
}, LocalPrefix {
  location: "http://purl.org/net/dbm/ontology#",
  content: include_str!("../prefixes/dbm.ttl"),
  name: "dbm",
  title: "DBM Ontology",
}, LocalPrefix {
  location: "http://ontology.cybershare.utep.edu/dbowl",
  content: include_str!("../prefixes/dbowl.ttl"),
  name: "dbowl",
  title: "Relational to Ontology Mapping Primitive",
}, LocalPrefix {
  location: "http://dbpedia.org/ontology/",
  content: include_str!("../prefixes/dbpedia-owl.ttl"),
  name: "dbpedia-owl",
  title: "The DBpedia Ontology",
}, LocalPrefix {
  location: "http://ontologi.es/doap-bugs#",
  content: include_str!("../prefixes/dbug.ttl"),
  name: "dbug",
  title: "DOAP Bugs",
}, LocalPrefix {
  location: "http://purl.org/dc/dcam/",
  content: include_str!("../prefixes/dcam.ttl"),
  name: "dcam",
  title: "DCMI Abstract Model",
}, LocalPrefix {
  location: "http://www.w3.org/ns/dcat",
  content: include_str!("../prefixes/dcat.ttl"),
  name: "dcat",
  title: "Data Catalog Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/dc/elements/1.1/",
  content: include_str!("../prefixes/dce.ttl"),
  name: "dce",
  title: "Dublin Core Metadata Element Set",
}, LocalPrefix {
  location: "http://purl.org/spar/datacite",
  content: include_str!("../prefixes/dcite.ttl"),
  name: "dcite",
  title: "The DataCite Ontology",
}, LocalPrefix {
  location: "http://ndl.go.jp/dcndl/terms/",
  content: include_str!("../prefixes/dcndl.ttl"),
  name: "dcndl",
  title: "NDL Metadata Terms",
}, LocalPrefix {
  location: "https://w3id.org/dco",
  content: include_str!("../prefixes/dco.ttl"),
  name: "dco",
  title: "domOS Common Ontology",
}, LocalPrefix {
  location: "http://purl.org/dc/terms/",
  content: include_str!("../prefixes/dcterms.ttl"),
  name: "dcterms",
  title: "DCMI Metadata Terms",
}, LocalPrefix {
  location: "http://purl.org/dc/dcmitype/",
  content: include_str!("../prefixes/dctype.ttl"),
  name: "dctype",
  title: "DCMI Type Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/arco/ontology/denotative-description",
  content: include_str!("../prefixes/ddesc.ttl"),
  name: "ddesc",
  title: "Denotative Description Ontology (ArCo network)",
}, LocalPrefix {
  location: "https://decision-ontology.googlecode.com/svn/trunk/decision.owl",
  content: include_str!("../prefixes/decision.ttl"),
  name: "decision",
  title: "Decision ontology",
}, LocalPrefix {
  location: "http://www.demcare.eu/ontologies/demlab.owl",
  content: include_str!("../prefixes/demlab.ttl"),
  name: "demlab",
  title: "Dem@Care Lab Ontology for Dementia Assessment",
}, LocalPrefix {
  location: "http://purl.org/spar/deo",
  content: include_str!("../prefixes/deo.ttl"),
  name: "deo",
  title: "The Discourse Elements Ontology",
}, LocalPrefix {
  location: "http://ontologi.es/doap-deps#",
  content: include_str!("../prefixes/deps.ttl"),
  name: "deps",
  title: "DOAP Dependencies",
}, LocalPrefix {
  location: "http://static.datafoodconsortium.org/ontologies/DFC_FullModel.owl",
  content: include_str!("../prefixes/dfc.ttl"),
  name: "dfc",
  title: "Data Food Consortium Ontology",
}, LocalPrefix {
  location: "http://static.datafoodconsortium.org/ontologies/DFC_BusinessOntology.owl",
  content: include_str!("../prefixes/dfc-b.ttl"),
  name: "dfc-b",
  title: "DFC Business Ontology",
}, LocalPrefix {
  location: "http://static.datafoodconsortium.org/ontologies/DFC_ProductGlossary.owl",
  content: include_str!("../prefixes/dfc-p.ttl"),
  name: "dfc-p",
  title: "A common vocabulary for digital food platforms (Product Glossary Part)",
}, LocalPrefix {
  location: "http://static.datafoodconsortium.org/ontologies/DFC_TechnicalOntology.owl",
  content: include_str!("../prefixes/dfc-t.ttl"),
  name: "dfc-t",
  title: "A common vocabulary for digital food platforms (Technical Part)",
}, LocalPrefix {
  location: "https://w3id.org/dingo/",
  content: include_str!("../prefixes/dg.ttl"),
  name: "dg",
  title: "DINGO Ontology",
}, LocalPrefix {
  location: "http://purl.org/healthcarevocab/v1",
  content: include_str!("../prefixes/dicom.ttl"),
  name: "dicom",
  title: "Healthcare metadata - DICOM ontology",
}, LocalPrefix {
  location: "https://w3id.org/dio",
  content: include_str!("../prefixes/dio.ttl"),
  name: "dio",
  title: "The Design Intent Ontology",
}, LocalPrefix {
  location: "http://rdf-vocabulary.ddialliance.org/discovery",
  content: include_str!("../prefixes/disco.ttl"),
  name: "disco",
  title: "DDI-RDF Discovery Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/dita/ns#",
  content: include_str!("../prefixes/dita.ttl"),
  name: "dita",
  title: "DITA RDF ontology",
}, LocalPrefix {
  location: "http://www.data-knowledge.org/dk/",
  content: include_str!("../prefixes/dk.ttl"),
  name: "dk",
  title: "The Data Knowledge Vocabulary",
}, LocalPrefix {
  location: "http://onto.dm2e.eu/schemas/dm2e",
  content: include_str!("../prefixes/dm2e.ttl"),
  name: "dm2e",
  title: "DM2E model",
}, LocalPrefix {
  location: "http://d-nb.info/standards/elementset/dnb",
  content: include_str!("../prefixes/dnbt.ttl"),
  name: "dnbt",
  title: "DNB Metadata Terms",
}, LocalPrefix {
  location: "http://usefulinc.com/ns/doap#",
  content: include_str!("../prefixes/doap.ttl"),
  name: "doap",
  title: "Description of a Project vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/2000/10/swap/pim/doc",
  content: include_str!("../prefixes/doc.ttl"),
  name: "doc",
  title: "Works, licences, derivatives and dependencies",
}, LocalPrefix {
  location: "http://purl.org/spar/doco",
  content: include_str!("../prefixes/doco.ttl"),
  name: "doco",
  title: "DoCO, the Document Components Ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/dso",
  content: include_str!("../prefixes/docso.ttl"),
  name: "docso",
  title: "Document Service Ontology",
}, LocalPrefix {
  location: "http://elite.polito.it/ontologies/dogont.owl",
  content: include_str!("../prefixes/dogont.ttl"),
  name: "dogont",
  title: "Ontology Modeling for Intelligent Domotic Environments",
}, LocalPrefix {
  location: "http://reference.data.gov.au/def/ont/dataset",
  content: include_str!("../prefixes/donto.ttl"),
  name: "donto",
  title: "Dataset Ontology",
}, LocalPrefix {
  location: "https://w3id.org/dot#",
  content: include_str!("../prefixes/dot.ttl"),
  name: "dot",
  title: "Damage Topology Ontology",
}, LocalPrefix {
  location: "http://purl.org/dpn",
  content: include_str!("../prefixes/dpn.ttl"),
  name: "dpn",
  title: "Data Provider Node ontology",
}, LocalPrefix {
  location: "http://purl.org/twc/dpo/ont/diabetes_pharmacology_ontology.ttl",
  content: include_str!("../prefixes/dpo.ttl"),
  name: "dpo",
  title: "Diabetes Pharmacology Ontology",
}, LocalPrefix {
  location: "http://promsns.org/def/decprov",
  content: include_str!("../prefixes/dprov.ttl"),
  name: "dprov",
  title: "Decision Provenance ontology (DecPROV)",
}, LocalPrefix {
  location: "https://w3id.org/dpv",
  content: include_str!("../prefixes/dpv.ttl"),
  name: "dpv",
  title: "Data Privacy Vocabulary (DPV)",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19115/2003/dataquality",
  content: include_str!("../prefixes/dq.ttl"),
  name: "dq",
  title: "OWL representation of ISO 19115 (Geographic Information - Metadata - Data quality package)",
}, LocalPrefix {
  location: "http://semwebquality.org/ontologies/dq-constraints",
  content: include_str!("../prefixes/dqc.ttl"),
  name: "dqc",
  title: "The Data Quality Constraints Library",
}, LocalPrefix {
  location: "http://purl.org/dqm-vocabulary/v1/dqm",
  content: include_str!("../prefixes/dqm.ttl"),
  name: "dqm",
  title: "The Data Quality Management Vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/dqv",
  content: include_str!("../prefixes/dqv.ttl"),
  name: "dqv",
  title: "Data Quality Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/swan/2.0/discourse-relationships/",
  content: include_str!("../prefixes/dr.ttl"),
  name: "dr",
  title: "Discourse relationships vocabulary",
}, LocalPrefix {
  location: "http://www.purl.org/drammar",
  content: include_str!("../prefixes/drama.ttl"),
  name: "drama",
  title: "Drammar: a comprehensive ontology of drama",
}, LocalPrefix {
  location: "http://vocab.data.gov/def/drm",
  content: include_str!("../prefixes/drm.ttl"),
  name: "drm",
  title: "Data Reference Model",
}, LocalPrefix {
  location: "http://purl.org/ctic/dcat#",
  content: include_str!("../prefixes/ds.ttl"),
  name: "ds",
  title: "Dataset Catalog Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/dsd",
  content: include_str!("../prefixes/dsd.ttl"),
  name: "dsd",
  title: "Description of a Data Source",
}, LocalPrefix {
  location: "http://purl.org/dsnotify/vocab/eventset/",
  content: include_str!("../prefixes/dsn.ttl"),
  name: "dsn",
  title: "DSNotify Eventsets: A vocabulary for change events in linked data sources",
}, LocalPrefix {
  location: "http://inference-web.org/2.0/ds.owl",
  content: include_str!("../prefixes/dso.ttl"),
  name: "dso",
  title: "Data Structure Ontology",
}, LocalPrefix {
  location: "http://w3id.org/dstv",
  content: include_str!("../prefixes/dstv.ttl"),
  name: "dstv",
  title: "DSTV:Steel Construction Ontology",
}, LocalPrefix {
  location: "http://w3id.org/dt",
  content: include_str!("../prefixes/dt.ttl"),
  name: "dt",
  title: "Data Template (DT) Ontology",
}, LocalPrefix {
  location: "http://cef.uv.es/lodroadtran18/def/transporte/dtx_srti",
  content: include_str!("../prefixes/dtx_srti.ttl"),
  name: "dtx_srti",
  title: "LOD SRTI DATEX II",
}, LocalPrefix {
  location: "http://www.linkedmodel.org/schema/dtype",
  content: include_str!("../prefixes/dtype.ttl"),
  name: "dtype",
  title: "Datatype Ontology",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/ont/dul/DUL.owl",
  content: include_str!("../prefixes/dul.ttl"),
  name: "dul",
  title: "DOLCE+DnS Ultralite",
}, LocalPrefix {
  location: "http://www.w3.org/ns/duv",
  content: include_str!("../prefixes/duv.ttl"),
  name: "duv",
  title: "Dataset Usage Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/ontology/dvia",
  content: include_str!("../prefixes/dvia.ttl"),
  name: "dvia",
  title: "The visualization vocabulary for LOD applications",
}, LocalPrefix {
  location: "http://archivi.ibc.regione.emilia-romagna.it/ontology/eac-cpf/",
  content: include_str!("../prefixes/eac-cpf.ttl"),
  name: "eac-cpf",
  title: "EAC-CPF Descriptions Ontology for Linked Archival Data:",
}, LocalPrefix {
  location: "http://www.w3.org/ns/earl",
  content: include_str!("../prefixes/earl.ttl"),
  name: "earl",
  title: "Evaluation and Report Language",
}, LocalPrefix {
  location: "http://linked.earth/ontology#",
  content: include_str!("../prefixes/earth.ttl"),
  name: "earth",
  title: "The Linked Earth Ontology",
}, LocalPrefix {
  location: "http://data.businessgraph.io/ontology#",
  content: include_str!("../prefixes/ebg.ttl"),
  name: "ebg",
  title: "euBusinessGraph ontology",
}, LocalPrefix {
  location: "http://www.ebu.ch/metadata/ontologies/ebucore/ebucore",
  content: include_str!("../prefixes/ebucore.ttl"),
  name: "ebucore",
  title: "EBU Ontology",
}, LocalPrefix {
  location: "https://vocab.eccenca.com/revision/",
  content: include_str!("../prefixes/eccrev.ttl"),
  name: "eccrev",
  title: "RDF changes and revisions vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/ecfo",
  content: include_str!("../prefixes/ecfo.ttl"),
  name: "ecfo",
  title: "The Emission Conversion Factor Ontology",
}, LocalPrefix {
  location: "http://www.eclap.eu/schema/eclap/",
  content: include_str!("../prefixes/eclap.ttl"),
  name: "eclap",
  title: "ECLAP, Performing Arts Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/ontology/ecpo",
  content: include_str!("../prefixes/ecpo.ttl"),
  name: "ecpo",
  title: "Enumeration and Chronology of Periodicals Ontology",
}, LocalPrefix {
  location: "http://erlangen-crm.org/current/",
  content: include_str!("../prefixes/ecrm.ttl"),
  name: "ecrm",
  title: "Erlangen CRM / OWL",
}, LocalPrefix {
  location: "http://ontology.cybershare.utep.edu/ELSEWeb/elseweb-edac.owl",
  content: include_str!("../prefixes/edac.ttl"),
  name: "edac",
  title: "ELSEWeb EDAC Ontology",
}, LocalPrefix {
  location: "https://purl.org/edifact/ontology",
  content: include_str!("../prefixes/edifact-o.ttl"),
  name: "edifact-o",
  title: "EDIFACT Ontology",
}, LocalPrefix {
  location: "http://www.europeana.eu/schemas/edm/",
  content: include_str!("../prefixes/edm.ttl"),
  name: "edm",
  title: "Europeana Data Model vocabulary",
}, LocalPrefix {
  location: "https://schema.edu.ee/",
  content: include_str!("../prefixes/edu.ttl"),
  name: "edu",
  title: "Education Ontology",
}, LocalPrefix {
  location: "http://ns.inria.fr/semed/eduprogression/",
  content: include_str!("../prefixes/edupro.ttl"),
  name: "edupro",
  title: "EduProgression Ontology",
}, LocalPrefix {
  location: "http://purl.org/eem",
  content: include_str!("../prefixes/eem.ttl"),
  name: "eem",
  title: "The EPCIS Event Model",
}, LocalPrefix {
  location: "https://w3id.org/eeo",
  content: include_str!("../prefixes/eeo.ttl"),
  name: "eeo",
  title: "Experimental Evaluation Ontology",
}, LocalPrefix {
  location: "https://opendata.aragon.es/def/ei2a/ei2a.owl",
  content: include_str!("../prefixes/ei2a.ttl"),
  name: "ei2a",
  title: "Aragon Interoperable Information Structure Ontology
EI2A",
}, LocalPrefix {
  location: "http://purl.org/ctic/sector-publico/elecciones",
  content: include_str!("../prefixes/elec.ttl"),
  name: "elec",
  title: "Vocabulary for Vote Results",
}, LocalPrefix {
  location: "http://data.europa.eu/eli/ontology",
  content: include_str!("../prefixes/eli.ttl"),
  name: "eli",
  title: "The European Legislation Identifier",
}, LocalPrefix {
  location: "https://w3id.org/emmo",
  content: include_str!("../prefixes/emmo.ttl"),
  name: "emmo",
  title: "Elementary Multiperspective Material Ontology (EMMO)",
}, LocalPrefix {
  location: "http://ns.inria.fr/emoca",
  content: include_str!("../prefixes/emotion.ttl"),
  name: "emotion",
  title: "Emotion Ontology for Context Awareness",
}, LocalPrefix {
  location: "http://purl.org/ctic/empleo/oferta",
  content: include_str!("../prefixes/emp.ttl"),
  name: "emp",
  title: "A vocabulary for jobs",
}, LocalPrefix {
  location: "http://labs.mondeca.com/vocab/endpointStatus",
  content: include_str!("../prefixes/ends.ttl"),
  name: "ends",
  title: "Vocabulary of endpoint status (availability, responseTime)",
}, LocalPrefix {
  location: "http://eprints.org/ontology/",
  content: include_str!("../prefixes/ep.ttl"),
  name: "ep",
  title: "EPrints Ontology",
}, LocalPrefix {
  location: "https://w3id.org/ep-plan",
  content: include_str!("../prefixes/eppl.ttl"),
  name: "eppl",
  title: "The EP-Plan ontology",
}, LocalPrefix {
  location: "https://data.nasa.gov/ontologies/atmonto/equipment#",
  content: include_str!("../prefixes/eqp.ttl"),
  name: "eqp",
  title: "Aircraft Equipment Vocabulary",
}, LocalPrefix {
  location: "http://data.europa.eu/949/",
  content: include_str!("../prefixes/era.ttl"),
  name: "era",
  title: "ERA vocabulary",
}, LocalPrefix {
  location: "http://data.europa.eu/esco/model",
  content: include_str!("../prefixes/esco.ttl"),
  name: "esco",
  title: "The ESCO ontology",
}, LocalPrefix {
  location: "http://purl.org/essglobal/vocab/",
  content: include_str!("../prefixes/essglobal.ttl"),
  name: "essglobal",
  title: "ESSGlobal Vocabulary",
}, LocalPrefix {
  location: "http://elite.polito.it/ontologies/eupont.owl",
  content: include_str!("../prefixes/eupont.ttl"),
  name: "eupont",
  title: "EUPont: an ontology for End User Programming of the IoT",
}, LocalPrefix {
  location: "http://data.europa.eu/s66#",
  content: include_str!("../prefixes/eurio.ttl"),
  name: "eurio",
  title: "EURIO: EUropean Research Information Ontology",
}, LocalPrefix {
  location: "http://purl.org/NET/c4dm/event.owl",
  content: include_str!("../prefixes/event.ttl"),
  name: "event",
  title: "The Event Ontology",
}, LocalPrefix {
  location: "http://purl.org/net/ns/ex",
  content: include_str!("../prefixes/ex.ttl"),
  name: "ex",
  title: "Example vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/2003/12/exif/ns",
  content: include_str!("../prefixes/exif.ttl"),
  name: "exif",
  title: "Exif data description vocabulary",
}, LocalPrefix {
  location: "http://w3id.org/emmo-maeo/maeo",
  content: include_str!("../prefixes/experts.ttl"),
  name: "experts",
  title: "MAEO - MarketPlace Agent and Expert Ontology",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19115/2003/extent",
  content: include_str!("../prefixes/ext.ttl"),
  name: "ext",
  title: "OWL representation of ISO 19115 (Geographic Information - Metadata - Extent package)",
}, LocalPrefix {
  location: "http://purl.org/spar/fabio",
  content: include_str!("../prefixes/fabio.ttl"),
  name: "fabio",
  title: "FRBR-aligned Bibliographic Ontology",
}, LocalPrefix {
  location: "http://biohackathon.org/resource/faldo",
  content: include_str!("../prefixes/faldo.ttl"),
  name: "faldo",
  title: "Feature Annotation Location Description Ontology",
}, LocalPrefix {
  location: "http://vocab.data.gov/def/fea",
  content: include_str!("../prefixes/fea.ttl"),
  name: "fea",
  title: "Federal Enterprise Architecture Vocabulary",
}, LocalPrefix {
  location: "http://w3id.org/vcb/fel#",
  content: include_str!("../prefixes/fel.ttl"),
  name: "fel",
  title: "A Fine-grained Entity Linking vocabulary",
}, LocalPrefix {
  location: "http://purl.org/iot/ontology/fiesta-iot",
  content: include_str!("../prefixes/fiesta-iot.ttl"),
  name: "fiesta-iot",
  title: "FIESTA-IoT Ontology",
}, LocalPrefix {
  location: "http://xmlns.com/foaf/0.1/",
  content: include_str!("../prefixes/foaf.ttl"),
  name: "foaf",
  title: "Friend of a Friend vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/fog",
  content: include_str!("../prefixes/fog.ttl"),
  name: "fog",
  title: "FOG: File Ontology for Geometry formats",
}, LocalPrefix {
  location: "https://w3id.org/seas/FeatureOfInterestOntology",
  content: include_str!("../prefixes/foio.ttl"),
  name: "foio",
  title: "The SEAS Feature of Interest ontology.",
}, LocalPrefix {
  location: "https://w3id.org/def/foo#",
  content: include_str!("../prefixes/foo.ttl"),
  name: "foo",
  title: "Forest Observatory Ontology (FOO)",
}, LocalPrefix {
  location: "http://data.lirmm.fr/ontologies/food",
  content: include_str!("../prefixes/food.ttl"),
  name: "food",
  title: "Food Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/TR/2003/PR-owl-guide-20031215/food",
  content: include_str!("../prefixes/fowl.ttl"),
  name: "fowl",
  title: "Food Ontology in OWL",
}, LocalPrefix {
  location: "http://iflastandards.info/ns/fr/frad/",
  content: include_str!("../prefixes/frad.ttl"),
  name: "frad",
  title: "FRAD model",
}, LocalPrefix {
  location: "http://purl.org/cerif/frapo/",
  content: include_str!("../prefixes/frapo.ttl"),
  name: "frapo",
  title: "Funding, Research Administration and Projects Ontology",
}, LocalPrefix {
  location: "http://streamreasoning.org/ontologies/frappe#",
  content: include_str!("../prefixes/frappe.ttl"),
  name: "frappe",
  title: "FraPPE: Frame, Pixel, Place, Event vocabulary",
}, LocalPrefix {
  location: "http://purl.org/vocab/frbr/core",
  content: include_str!("../prefixes/frbr.ttl"),
  name: "frbr",
  title: "Expression of Core FRBR Concepts in RDF",
}, LocalPrefix {
  location: "http://purl.org/vocab/frbr/extended",
  content: include_str!("../prefixes/frbre.ttl"),
  name: "frbre",
  title: "Extended FRBR",
}, LocalPrefix {
  location: "http://iflastandards.info/ns/fr/frbr/frbrer/",
  content: include_str!("../prefixes/frbrer.ttl"),
  name: "frbrer",
  title: "FRBRer model",
}, LocalPrefix {
  location: "http://www.w3.org/2004/09/fresnel",
  content: include_str!("../prefixes/fresnel.ttl"),
  name: "fresnel",
  title: "Fresnel Lens and Format Core Vocabulary",
}, LocalPrefix {
  location: "http://data.ordnancesurvey.co.uk/ontology/50kGazetteer/",
  content: include_str!("../prefixes/g50k.ttl"),
  name: "g50k",
  title: "50K Gazetteer Vocabulary",
}, LocalPrefix {
  location: "http://data.totl.net/game/",
  content: include_str!("../prefixes/game.ttl"),
  name: "game",
  title: "TotL Game Ontology",
}, LocalPrefix {
  location: "http://www.oegov.org/core/owl/gc",
  content: include_str!("../prefixes/gc.ttl"),
  name: "gc",
  title: "oeGOV Government Core Ontology",
}, LocalPrefix {
  location: "http://ontology.eil.utoronto.ca/GCI/Foundation/GCI-Foundation.owl",
  content: include_str!("../prefixes/gci.ttl"),
  name: "gci",
  title: "Global City Indicator Foundation Ontology",
}, LocalPrefix {
  location: "https://w3id.org/GConsent",
  content: include_str!("../prefixes/gcon.ttl"),
  name: "gcon",
  title: "GConsent - a consent ontology based on the GDPR",
}, LocalPrefix {
  location: "http://vocab.data.gov/gd",
  content: include_str!("../prefixes/gd.ttl"),
  name: "gd",
  title: "Government Data Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/GDPRov",
  content: include_str!("../prefixes/gdprov.ttl"),
  name: "gdprov",
  title: "The GDPR Provenance ontology",
}, LocalPrefix {
  location: "https://w3id.org/GDPRtEXT",
  content: include_str!("../prefixes/gdprt.ttl"),
  name: "gdprt",
  title: "GDPR text EXTensions",
}, LocalPrefix {
  location: "http://purl.org/gen/0.1#",
  content: include_str!("../prefixes/gen.ttl"),
  name: "gen",
  title: "Vocabulary for Linked Genealogical Data",
}, LocalPrefix {
  location: "http://www.w3.org/2003/01/geo/wgs84_pos",
  content: include_str!("../prefixes/geo.ttl"),
  name: "geo",
  title: "WGS84 Geo Positioning",
}, LocalPrefix {
  location: "http://vocab.lenka.no/geo-deling",
  content: include_str!("../prefixes/geod.ttl"),
  name: "geod",
  title: "Administrative vocabulary for Norway",
}, LocalPrefix {
  location: "http://www.mindswap.org/2003/owl/geo/geoFeatures20040307.owl",
  content: include_str!("../prefixes/geof.ttl"),
  name: "geof",
  title: "Geo Features",
}, LocalPrefix {
  location: "http://data.ign.fr/def/geofla",
  content: include_str!("../prefixes/geofla.ttl"),
  name: "geofla",
  title: "Ontology of administrative units at IGN-France",
}, LocalPrefix {
  location: "http://data.ign.fr/def/geometrie",
  content: include_str!("../prefixes/geom.ttl"),
  name: "geom",
  title: "Ontology for geometry",
}, LocalPrefix {
  location: "http://aims.fao.org/aos/geopolitical.owl",
  content: include_str!("../prefixes/geop.ttl"),
  name: "geop",
  title: "FAO Geopolitical Ontology",
}, LocalPrefix {
  location: "http://rdf.geospecies.org/ont/geospecies",
  content: include_str!("../prefixes/geosp.ttl"),
  name: "geosp",
  title: "GeoSpecies Ontology",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19109/2005/feature",
  content: include_str!("../prefixes/gf.ttl"),
  name: "gf",
  title: "OWL representation of ISO 19109 (General Feature Model)",
}, LocalPrefix {
  location: "https://www.gleif.org/ontology/L1/",
  content: include_str!("../prefixes/gleif-L1.ttl"),
  name: "gleif-L1",
  title: "Global Legal Entity Identifier Foundation Level 1 Ontology - Who Is Who",
}, LocalPrefix {
  location: "https://www.gleif.org/ontology/L2/",
  content: include_str!("../prefixes/gleif-L2.ttl"),
  name: "gleif-L2",
  title: "Global Legal Entity Identifier Foundation Level 2 Ontology - Who Owns Whom",
}, LocalPrefix {
  location: "https://www.gleif.org/ontology/Base/",
  content: include_str!("../prefixes/gleif-base.ttl"),
  name: "gleif-base",
  title: "Global Legal Entity Identifier Foundation Base Ontology",
}, LocalPrefix {
  location: "https://www.gleif.org/ontology/EntityLegalForm/",
  content: include_str!("../prefixes/gleif-elf.ttl"),
  name: "gleif-elf",
  title: "Entity Legal Form Ontology",
}, LocalPrefix {
  location: "https://www.gleif.org/ontology/Geocoding/",
  content: include_str!("../prefixes/gleif-geo.ttl"),
  name: "gleif-geo",
  title: "Global Legal Entity Identifier Foundation Geocoding Ontology",
}, LocalPrefix {
  location: "https://www.gleif.org/ontology/RegistrationAuthority/",
  content: include_str!("../prefixes/gleif-ra.ttl"),
  name: "gleif-ra",
  title: "Global Legal Entity Identifier Foundation Registration Authority Ontology",
}, LocalPrefix {
  location: "https://www.gleif.org/ontology/ReportingException/",
  content: include_str!("../prefixes/gleif-repex.ttl"),
  name: "gleif-repex",
  title: "Global Legal Entity Identifier Foundation Reporting Exception Ontology",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19107/2003/geometry",
  content: include_str!("../prefixes/gm.ttl"),
  name: "gm",
  title: "OWL representation of ISO 19107 (Geographic Information)",
}, LocalPrefix {
  location: "http://www.opengis.net/ont/gml",
  content: include_str!("../prefixes/gml.ttl"),
  name: "gml",
  title: "OGC Geometry",
}, LocalPrefix {
  location: "http://www.geonames.org/ontology",
  content: include_str!("../prefixes/gn.ttl"),
  name: "gn",
  title: "The Geonames ontology",
}, LocalPrefix {
  location: "http://d-nb.info/standards/elementset/gnd#",
  content: include_str!("../prefixes/gndo.ttl"),
  name: "gndo",
  title: "GND Ontology",
}, LocalPrefix {
  location: "http://purl.org/linguistics/gold",
  content: include_str!("../prefixes/gold.ttl"),
  name: "gold",
  title: "General Ontology for Linguistic Description",
}, LocalPrefix {
  location: "https://w3id.org/gom",
  content: include_str!("../prefixes/gom.ttl"),
  name: "gom",
  title: "GOM: Geometry Metadata Ontology",
}, LocalPrefix {
  location: "http://gov.genealogy.net/ontology.owl",
  content: include_str!("../prefixes/gov.ttl"),
  name: "gov",
  title: "Ontology for modelling historic administrative information.",
}, LocalPrefix {
  location: "http://purl.org/goodrelations/v1",
  content: include_str!("../prefixes/gr.ttl"),
  name: "gr",
  title: "The GoodRelations Ontology for Semantic Web-based E-Commerce",
}, LocalPrefix {
  location: "http://www.w3.org/2003/g/data-view",
  content: include_str!("../prefixes/grddl.ttl"),
  name: "grddl",
  title: "Gleaning Resource Descriptions from Dialects of Languages Vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/2006/gen/ont",
  content: include_str!("../prefixes/gso.ttl"),
  name: "gso",
  title: "Generic Specific Ontology",
}, LocalPrefix {
  location: "http://www.opengis.net/ont/geosparql",
  content: include_str!("../prefixes/gsp.ttl"),
  name: "gsp",
  title: "OGC GeoSPARQL",
}, LocalPrefix {
  location: "http://vocab.gtfs.org/terms#",
  content: include_str!("../prefixes/gtfs.ttl"),
  name: "gtfs",
  title: "General Transit Feed Specification",
}, LocalPrefix {
  location: "https://www.goudatijdmachine.nl/def",
  content: include_str!("../prefixes/gtm.ttl"),
  name: "gtm",
  title: "Gouda Time Machine Ontology",
}, LocalPrefix {
  location: "http://resource.geosciml.org/ontology/timescale/gts",
  content: include_str!("../prefixes/gts.ttl"),
  name: "gts",
  title: "Geologic Timescale model",
}, LocalPrefix {
  location: "http://purl.org/nemo/gufo#",
  content: include_str!("../prefixes/gufo.ttl"),
  name: "gufo",
  title: "gUFO: A Lightweight Implementation of the Unified Foundational Ontology (UFO)",
}, LocalPrefix {
  location: "http://vocab.getty.edu/ontology",
  content: include_str!("../prefixes/gvp.ttl"),
  name: "gvp",
  title: "Getty Vocabulary Program ontology",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19150/-2/2012/basic",
  content: include_str!("../prefixes/h2o.ttl"),
  name: "h2o",
  title: "Ontology for conversion of ISO/TC 211",
}, LocalPrefix {
  location: "http://sensormeasurement.appspot.com/ont/home/homeActivity#",
  content: include_str!("../prefixes/ha.ttl"),
  name: "ha",
  title: "Home Activity",
}, LocalPrefix {
  location: "https://www.w3.org/2019/wot/hypermedia#",
  content: include_str!("../prefixes/hctl.ttl"),
  name: "hctl",
  title: "Hypermedia Controls Ontology",
}, LocalPrefix {
  location: "http://www.samos.gr/ontologies/helpdeskOnto.owl",
  content: include_str!("../prefixes/hdo.ttl"),
  name: "hdo",
  title: "HelpDesk support Ontology",
}, LocalPrefix {
  location: "https://w3id.org/HHT",
  content: include_str!("../prefixes/hht.ttl"),
  name: "hht",
  title: "Historical Hierarchical Territories",
}, LocalPrefix {
  location: "http://purl.org/net/hifm/ontology#",
  content: include_str!("../prefixes/hifm.ttl"),
  name: "hifm",
  title: "HIFM Ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/holding",
  content: include_str!("../prefixes/holding.ttl"),
  name: "holding",
  title: "Holding Ontology",
}, LocalPrefix {
  location: "http://purl.org/holy/ns#",
  content: include_str!("../prefixes/holy.ttl"),
  name: "holy",
  title: "Hydrogen Ontology",
}, LocalPrefix {
  location: "http://vocab.data.gov/hosp",
  content: include_str!("../prefixes/hosp.ttl"),
  name: "hosp",
  title: "Hospital Vocabulary",
}, LocalPrefix {
  location: "http://w3id.org/emmo-hpo/hpo",
  content: include_str!("../prefixes/hpo.ttl"),
  name: "hpo",
  title: "Hyperdimensional Polymer Ontology",
}, LocalPrefix {
  location: "https://w3id.org/hpont",
  content: include_str!("../prefixes/hpont.ttl"),
  name: "hpont",
  title: "The Heat Pump Ontology (HPOnt).",
}, LocalPrefix {
  location: "http://iserve.kmi.open.ac.uk/ns/hrests",
  content: include_str!("../prefixes/hr.ttl"),
  name: "hr",
  title: "hRESTS Ontology",
}, LocalPrefix {
  location: "http://vcharpenay.github.io/hto/hto.xml",
  content: include_str!("../prefixes/hto.ttl"),
  name: "hto",
  title: "Haystack Tagging Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2011/http",
  content: include_str!("../prefixes/http.ttl"),
  name: "http",
  title: "HTTP in RDF",
}, LocalPrefix {
  location: "https://www.auto.tuwien.ac.at/downloads/thinkhome/ontology/WeatherOntology.owl",
  content: include_str!("../prefixes/hw.ttl"),
  name: "hw",
  title: "Home Weather",
}, LocalPrefix {
  location: "http://www.w3.org/ns/hydra/core",
  content: include_str!("../prefixes/hydra.ttl"),
  name: "hydra",
  title: "The Hydra Core Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/iadopt/ont",
  content: include_str!("../prefixes/iadopt.ttl"),
  name: "iadopt",
  title: "I-ADOPT Framework ontology",
}, LocalPrefix {
  location: "https://privatealpha.com/ontology/ibis/1#",
  content: include_str!("../prefixes/ibis.ttl"),
  name: "ibis",
  title: "IBIS Vocabulary",
}, LocalPrefix {
  location: "http://ontology.eil.utoronto.ca/icontact.owl",
  content: include_str!("../prefixes/ic.ttl"),
  name: "ic",
  title: "International Contact Ontology: Addresses, phone numbers and emails.",
}, LocalPrefix {
  location: "https://w3id.org/isCharacterisedBy",
  content: include_str!("../prefixes/icb.ttl"),
  name: "icb",
  title: "isCharacterisedBy ontology design pattern",
}, LocalPrefix {
  location: "https://w3id.org/iddo",
  content: include_str!("../prefixes/iddo.ttl"),
  name: "iddo",
  title: "The Interconnected Data Dictionary Ontology (IDDO)",
}, LocalPrefix {
  location: "http://rdf.insee.fr/def/demo",
  content: include_str!("../prefixes/idemo.ttl"),
  name: "idemo",
  title: "Demographic ontology from the French Statistics Institute",
}, LocalPrefix {
  location: "http://www.identity.org/ontologies/identity.owl",
  content: include_str!("../prefixes/identity.ttl"),
  name: "identity",
  title: "Ontology of digital identity.",
}, LocalPrefix {
  location: "https://w3id.org/idsa/core",
  content: include_str!("../prefixes/ids.ttl"),
  name: "ids",
  title: "IDS Information Model",
}, LocalPrefix {
  location: "https://w3id.org/ifc/IFC4_ADD1",
  content: include_str!("../prefixes/ifc.ttl"),
  name: "ifc",
  title: "IFC4_ADD1",
}, LocalPrefix {
  location: "http://rdf.insee.fr/def/geo",
  content: include_str!("../prefixes/igeo.ttl"),
  name: "igeo",
  title: "French Statistical ontology for geolocation",
}, LocalPrefix {
  location: "http://data.ign.fr/def/ignf",
  content: include_str!("../prefixes/ignf.ttl"),
  name: "ignf",
  title: "Ontology of coordinates reference systems",
}, LocalPrefix {
  location: "http://imgpedia.dcc.uchile.cl/ontology",
  content: include_str!("../prefixes/imo.ttl"),
  name: "imo",
  title: "The IMGpedia Ontology",
}, LocalPrefix {
  location: "http://vocab.resc.info/incident",
  content: include_str!("../prefixes/incident.ttl"),
  name: "incident",
  title: "Vocabulary to describe incident response by emergency services",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/cp/owl/informationrealization.owl",
  content: include_str!("../prefixes/infor.ttl"),
  name: "infor",
  title: "Information Realization",
}, LocalPrefix {
  location: "http://purl.org/innovation/ns",
  content: include_str!("../prefixes/inno.ttl"),
  name: "inno",
  title: "Ontology for Innovation",
}, LocalPrefix {
  location: "http://reference.data.gov.uk/def/intervals",
  content: include_str!("../prefixes/interval.ttl"),
  name: "interval",
  title: "Intervals Ontology",
}, LocalPrefix {
  location: "https://w3id.org/lso/intro/beta202408",
  content: include_str!("../prefixes/intro.ttl"),
  name: "intro",
  title: "INTRO: the intertextual, interpictorial, and intermedial relations ontology",
}, LocalPrefix {
  location: "http://w3id.org/ioc",
  content: include_str!("../prefixes/ioc.ttl"),
  name: "ioc",
  title: "IOC: Internet of Construction Ontology",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/ont/dul/IOLite.owl",
  content: include_str!("../prefixes/iol.ttl"),
  name: "iol",
  title: "Information Objects ontology",
}, LocalPrefix {
  location: "http://purl.oclc.org/NET/UNIS/fiware/iot-lite#",
  content: include_str!("../prefixes/iot-lite.ttl"),
  name: "iot-lite",
  title: "Iot-lite ontology",
}, LocalPrefix {
  location: "http://www.irit.fr/recherches/MELODI/ontologies/IoT-O",
  content: include_str!("../prefixes/ioto.ttl"),
  name: "ioto",
  title: "IoT-O",
}, LocalPrefix {
  location: "http://purl.org/iot/vocab/iot-taxonomy-lite#",
  content: include_str!("../prefixes/iottaxolite.ttl"),
  name: "iottaxolite",
  title: "The IoTTaxonomy-lite Taxonomy",
}, LocalPrefix {
  location: "http://purl.org/ipo/core",
  content: include_str!("../prefixes/ipo.ttl"),
  name: "ipo",
  title: "IPO - Issue Procedure Ontology",
}, LocalPrefix {
  location: "http://ontology.ethereal.cz/irao",
  content: include_str!("../prefixes/irao.ttl"),
  name: "irao",
  title: "Informatics Research Artifacts Ontology",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/ont/web/irw.owl",
  content: include_str!("../prefixes/irw.ttl"),
  name: "irw",
  title: "The Identity of Resources on the Web ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/is/core#",
  content: include_str!("../prefixes/is.ttl"),
  name: "is",
  title: "Info Service Ontology",
}, LocalPrefix {
  location: "http://iflastandards.info/ns/isbd/elements/",
  content: include_str!("../prefixes/isbd.ttl"),
  name: "isbd",
  title: "ISBD elements",
}, LocalPrefix {
  location: "http://purl.org/iso25964/skos-thes",
  content: include_str!("../prefixes/iso-thes.ttl"),
  name: "iso-thes",
  title: "ISO 25964 SKOS extension",
}, LocalPrefix {
  location: "http://ontology.eil.utoronto.ca/ISO37120.owl",
  content: include_str!("../prefixes/iso37120.ttl"),
  name: "iso37120",
  title: "ISO 37120 indicator URIs",
}, LocalPrefix {
  location: "http://reference.data.gov.au/def/ont/iso19160-1-address",
  content: include_str!("../prefixes/isoadr.ttl"),
  name: "isoadr",
  title: "ISO19160-1:2015 Address ontology",
}, LocalPrefix {
  location: "https://w3id.org/isoprops",
  content: include_str!("../prefixes/isoprops.ttl"),
  name: "isoprops",
  title: "ISO 23386 Property Ontology (ISOProps)",
}, LocalPrefix {
  location: "http://dati.isprambiente.it/ontology/core#",
  content: include_str!("../prefixes/ispra.ttl"),
  name: "ispra",
  title: "Ispra Ontology",
}, LocalPrefix {
  location: "https://data.istex.fr/ontology/istex#",
  content: include_str!("../prefixes/istex.ttl"),
  name: "istex",
  title: "Istex ontology for scholarly documents and extracted entities",
}, LocalPrefix {
  location: "http://spi-fm.uca.es/spdef/models/genericTools/itm/1.0",
  content: include_str!("../prefixes/itm.ttl"),
  name: "itm",
  title: "Issue Tracking tool Model",
}, LocalPrefix {
  location: "http://ontology.it/itsmo/v1",
  content: include_str!("../prefixes/itsmo.ttl"),
  name: "itsmo",
  title: "IT Service Management Ontology",
}, LocalPrefix {
  location: "http://www.ivoa.net/rdf/messenger",
  content: include_str!("../prefixes/ivoam.ttl"),
  name: "ivoam",
  title: "Messengers",
}, LocalPrefix {
  location: "https://www.w3.org/2019/wot/json-schema#",
  content: include_str!("../prefixes/jsonsc.ttl"),
  name: "jsonsc",
  title: "JSON Schema in RDF",
}, LocalPrefix {
  location: "http://w3id.org/charta77/jup",
  content: include_str!("../prefixes/jup.ttl"),
  name: "jup",
  title: "Ontology of Building Accessibility",
}, LocalPrefix {
  location: "http://rdfs.co/juso/",
  content: include_str!("../prefixes/juso.ttl"),
  name: "juso",
  title: "Juso Ontology",
}, LocalPrefix {
  location: "http://rdfs.co/juso/kr/",
  content: include_str!("../prefixes/juso.kr.ttl"),
  name: "juso.kr",
  title: "South Korea Extension to Juso Ontology",
}, LocalPrefix {
  location: "http://kdo.render-project.eu/kdo#",
  content: include_str!("../prefixes/kdo.ttl"),
  name: "kdo",
  title: "The Knowledge Diversity Ontology",
}, LocalPrefix {
  location: "http://linkeddata.center/kees/v1",
  content: include_str!("../prefixes/kees.ttl"),
  name: "kees",
  title: "KEES Ontology",
}, LocalPrefix {
  location: "http://purl.org/NET/c4dm/keys.owl",
  content: include_str!("../prefixes/keys.ttl"),
  name: "keys",
  title: "Keys Ontology",
}, LocalPrefix {
  location: "http://kgc.knowledge-graph.jp/ontology/kgc.owl",
  content: include_str!("../prefixes/kgc.ttl"),
  name: "kgc",
  title: "KGRC Ontology",
}, LocalPrefix {
  location: "http://www.disit.org/km4city/schema",
  content: include_str!("../prefixes/km4c.ttl"),
  name: "km4c",
  title: "km4city, the DISIT Knowledge Model for City and Mobility",
}, LocalPrefix {
  location: "http://purl.org/net/vocab/2004/03/label",
  content: include_str!("../prefixes/label.ttl"),
  name: "label",
  title: "label",
}, LocalPrefix {
  location: "http://lawd.info/ontology/",
  content: include_str!("../prefixes/lawd.ttl"),
  name: "lawd",
  title: "Linking Ancient World Data Ontology",
}, LocalPrefix {
  location: "http://semweb.mmlab.be/ns/linkedconnections#Ontology",
  content: include_str!("../prefixes/lc.ttl"),
  name: "lc",
  title: "The Linked Connections ontology",
}, LocalPrefix {
  location: "http://purl.org/vocab/lifecycle/schema",
  content: include_str!("../prefixes/lcy.ttl"),
  name: "lcy",
  title: "Lifecycle Schema",
}, LocalPrefix {
  location: "http://www.w3.org/ns/ldp#",
  content: include_str!("../prefixes/ldp.ttl"),
  name: "ldp",
  title: "Linked Data Platform",
}, LocalPrefix {
  location: "http://purl.oclc.org/NET/ldr/ns#",
  content: include_str!("../prefixes/ldr.ttl"),
  name: "ldr",
  title: "Linked Data Rights (LDR)",
}, LocalPrefix {
  location: "http://linked.opendata.cz/ontology/ldvm/",
  content: include_str!("../prefixes/ldvm.ttl"),
  name: "ldvm",
  title: "Vocabulary for Linked Data Visualization Model",
}, LocalPrefix {
  location: "http://lemon-model.net/lemon",
  content: include_str!("../prefixes/lemon.ttl"),
  name: "lemon",
  title: "LExicon Model for ONtologies",
}, LocalPrefix {
  location: "http://www.w3.org/ns/lemon/decomp",
  content: include_str!("../prefixes/lexdcp.ttl"),
  name: "lexdcp",
  title: "Lexicon Model for Ontologies - Decomp",
}, LocalPrefix {
  location: "http://www.lexinfo.net/ontology/2.0/lexinfo",
  content: include_str!("../prefixes/lexinfo.ttl"),
  name: "lexinfo",
  title: "LexInfo Ontology",
}, LocalPrefix {
  location: "http://linkedgeodata.org/ontology",
  content: include_str!("../prefixes/lgdo.ttl"),
  name: "lgdo",
  title: "LinkedGeoData ontology",
}, LocalPrefix {
  location: "https://w3id.org/legalhtml/ov",
  content: include_str!("../prefixes/lh.ttl"),
  name: "lh",
  title: "LegalHTML Ontology",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19115/2003/lineage",
  content: include_str!("../prefixes/li.ttl"),
  name: "li",
  title: "OWL representation of ISO 19115 (Geographic Information - Metadata - Lineage package)",
}, LocalPrefix {
  location: "http://purl.org/library/",
  content: include_str!("../prefixes/lib.ttl"),
  name: "lib",
  title: "Library extension of schema.org",
}, LocalPrefix {
  location: "http://www.irit.fr/recherches/MELODI/ontologies/IoT-Lifecycle",
  content: include_str!("../prefixes/lifecycle.ttl"),
  name: "lifecycle",
  title: "IoT-Lifecycle",
}, LocalPrefix {
  location: "http://www.w3.org/ns/lemon/lime",
  content: include_str!("../prefixes/lime.ttl"),
  name: "lime",
  title: "Lexicon Model for Ontologies - LIngusitic MEtadata (LIME)",
}, LocalPrefix {
  location: "http://purl.org/limo-ontology/limo/",
  content: include_str!("../prefixes/limo.ttl"),
  name: "limo",
  title: "Linked Statistical Models Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/LiMo/0.1#",
  content: include_str!("../prefixes/limoo.ttl"),
  name: "limoo",
  title: "License Model Ontology",
}, LocalPrefix {
  location: "https://w3id.org/vocab/lingvoj",
  content: include_str!("../prefixes/lingvo.ttl"),
  name: "lingvo",
  title: "The Lingvoj Ontology",
}, LocalPrefix {
  location: "http://purl.org/net/lio",
  content: include_str!("../prefixes/lio.ttl"),
  name: "lio",
  title: "Lightweight Image Ontology",
}, LocalPrefix {
  location: "http://www.linklion.org/ontology",
  content: include_str!("../prefixes/llont.ttl"),
  name: "llont",
  title: "LinkLion - the Link Discovery Portal",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/ont/lmm/LMM_L1.owl",
  content: include_str!("../prefixes/lmm1.ttl"),
  name: "lmm1",
  title: "Lexical MetaModel Level 1",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/ont/lmm/LMM_L2.owl",
  content: include_str!("../prefixes/lmm2.ttl"),
  name: "lmm2",
  title: "Lexical MetaModel Level 2",
}, LocalPrefix {
  location: "http://purl.org/ctic/infraestructuras/localizacion",
  content: include_str!("../prefixes/loc.ttl"),
  name: "loc",
  title: "Location Vocabulary",
}, LocalPrefix {
  location: "http://data.archiveshub.ac.uk/def/",
  content: include_str!("../prefixes/locah.ttl"),
  name: "locah",
  title: "The LOCAH RDF Vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/locn",
  content: include_str!("../prefixes/locn.ttl"),
  name: "locn",
  title: "ISA Programme Location Core Vocabulary",
}, LocalPrefix {
  location: "http://linkedevents.org/ontology/",
  content: include_str!("../prefixes/lode.ttl"),
  name: "lode",
  title: "Linking Open Descriptions of Events",
}, LocalPrefix {
  location: "http://www.w3.org/2000/10/swap/log",
  content: include_str!("../prefixes/log.ttl"),
  name: "log",
  title: "SWAP Logic Ontology",
}, LocalPrefix {
  location: "https://w3id.org/loin",
  content: include_str!("../prefixes/loin.ttl"),
  name: "loin",
  title: "Level of Information Need (LOIN) Ontology",
}, LocalPrefix {
  location: "http://data.opendiscoveryspace.eu/lom_ontology_ods.owl",
  content: include_str!("../prefixes/lom.ttl"),
  name: "lom",
  title: "Learning Object Metadata Ontology",
}, LocalPrefix {
  location: "http://sparql.sstu.ru:3030/speciality/",
  content: include_str!("../prefixes/losp.ttl"),
  name: "losp",
  title: "Linked open specialities RF",
}, LocalPrefix {
  location: "http://loted.eu/ontology",
  content: include_str!("../prefixes/loted.ttl"),
  name: "loted",
  title: "LOTED ontology",
}, LocalPrefix {
  location: "http://linkedscience.org/lsc/ns#",
  content: include_str!("../prefixes/lsc.ttl"),
  name: "lsc",
  title: "Linked Science Core Vocabulary",
}, LocalPrefix {
  location: "http://ontology.cybershare.utep.edu/ELSEWeb/elseweb-lifemapper.owl",
  content: include_str!("../prefixes/lslife.ttl"),
  name: "lslife",
  title: "ELSEWeb Lifemapper Ontology",
}, LocalPrefix {
  location: "http://ontology.cybershare.utep.edu/ELSEWeb/mappings/elseweb-mappings.owl",
  content: include_str!("../prefixes/lsmap.ttl"),
  name: "lsmap",
  title: "ELSEWeb Mappings Ontology",
}, LocalPrefix {
  location: "http://lsq.aksw.org/vocab",
  content: include_str!("../prefixes/lsq.ttl"),
  name: "lsq",
  title: "The Linked SPARQL Queries Vocabulary (LSQ(V))",
}, LocalPrefix {
  location: "http://ontology.cybershare.utep.edu/ELSEWeb/elseweb-data.owl",
  content: include_str!("../prefixes/lsweb.ttl"),
  name: "lsweb",
  title: "ELSEWeb Data Ontology",
}, LocalPrefix {
  location: "http://ontology.cybershare.utep.edu/ELSEWeb/elseweb-modelling.owl",
  content: include_str!("../prefixes/lswmo.ttl"),
  name: "lswmo",
  title: "ELSEWeb Modelling Ontology",
}, LocalPrefix {
  location: "http://ontology.cybershare.utep.edu/ELSEWeb/elseweb-lifemapper-parameters.owl",
  content: include_str!("../prefixes/lswpm.ttl"),
  name: "lswpm",
  title: "ELSEWeb Lifemapper Parameters Ontology",
}, LocalPrefix {
  location: "http://ns.inria.fr/ludo",
  content: include_str!("../prefixes/ludo.ttl"),
  name: "ludo",
  title: "Ludo Ontology",
}, LocalPrefix {
  location: "http://ns.inria.fr/ludo/v1/gamemodel#",
  content: include_str!("../prefixes/ludo-gm.ttl"),
  name: "ludo-gm",
  title: "Ludo Game Model Ontology",
}, LocalPrefix {
  location: "http://ns.inria.fr/ludo/v1/gamepresentation#",
  content: include_str!("../prefixes/ludo-gp.ttl"),
  name: "ludo-gp",
  title: "Ludo - Game Presentation",
}, LocalPrefix {
  location: "http://ns.inria.fr/ludo/v1/virtualcontext#",
  content: include_str!("../prefixes/ludo-vc.ttl"),
  name: "ludo-vc",
  title: "Ludo - Virtual Context",
}, LocalPrefix {
  location: "http://ns.inria.fr/ludo/v1/xapi",
  content: include_str!("../prefixes/ludo-xapi.ttl"),
  name: "ludo-xapi",
  title: "Experience API (xAPI)",
}, LocalPrefix {
  location: "http://purl.org/lobid/lv",
  content: include_str!("../prefixes/lv.ttl"),
  name: "lv",
  title: "lobid vocab",
}, LocalPrefix {
  location: "http://lexvo.org/ontology",
  content: include_str!("../prefixes/lvont.ttl"),
  name: "lvont",
  title: "Lexvo.org Ontology",
}, LocalPrefix {
  location: "http://purl.org/linkingyou/",
  content: include_str!("../prefixes/lyou.ttl"),
  name: "lyou",
  title: "Linking-you vocabulary",
}, LocalPrefix {
  location: "http://purl.org/iot/vocab/m3-lite#",
  content: include_str!("../prefixes/m3lite.ttl"),
  name: "m3lite",
  title: "The Machine-to-Machine Measurement (M3) Lite Ontology",
}, LocalPrefix {
  location: "http://w3id.org/nfdi4ing/metadata4ing#",
  content: include_str!("../prefixes/m4i.ttl"),
  name: "m4i",
  title: "Metadata4Ing: An ontology for describing the generation of research data within a scientific activity.",
}, LocalPrefix {
  location: "http://www.w3.org/ns/ma-ont",
  content: include_str!("../prefixes/ma-ont.ttl"),
  name: "ma-ont",
  title: "Ontology for Media Resources",
}, LocalPrefix {
  location: "http://www.loc.gov/mads/rdf/v1",
  content: include_str!("../prefixes/mads.ttl"),
  name: "mads",
  title: "Metadata Authority Description Schema",
}, LocalPrefix {
  location: "http://www.gsi.dit.upm.es/ontologies/marl/ns",
  content: include_str!("../prefixes/marl.ttl"),
  name: "marl",
  title: "Marl Ontology Specification",
}, LocalPrefix {
  location: "http://securitytoolbox.appspot.com/MASO",
  content: include_str!("../prefixes/maso.ttl"),
  name: "maso",
  title: "Mobile Agents Security",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19115/2003/metadata",
  content: include_str!("../prefixes/md.ttl"),
  name: "md",
  title: "OWL representation of ISO 19115",
}, LocalPrefix {
  location: "https://w3id.org/multidimensional-interface/ontology",
  content: include_str!("../prefixes/mdi.ttl"),
  name: "mdi",
  title: "RDF vocabulary to describe a Multidimensional Interface.",
}, LocalPrefix {
  location: "https://w3id.org/mdo/full/",
  content: include_str!("../prefixes/mdo.ttl"),
  name: "mdo",
  title: "Materials Design Ontology - Full",
}, LocalPrefix {
  location: "https://w3id.org/mdo/calculation/",
  content: include_str!("../prefixes/mdo-calc.ttl"),
  name: "mdo-calc",
  title: "Materials Design Ontology - Calculation Module",
}, LocalPrefix {
  location: "https://w3id.org/mdo/structure/",
  content: include_str!("../prefixes/mdo-struc.ttl"),
  name: "mdo-struc",
  title: "Materials Design Ontology - Structure Module",
}, LocalPrefix {
  location: "https://w3id.org/mdo/provenance/",
  content: include_str!("../prefixes/mdoprov.ttl"),
  name: "mdoprov",
  title: "Materials Design Ontology - Provenance Module",
}, LocalPrefix {
  location: "http://semanticturkey.uniroma2.it/ns/mdr",
  content: include_str!("../prefixes/mdr.ttl"),
  name: "mdr",
  title: "The Semantic Turkey metadata registry ontology",
}, LocalPrefix {
  location: "http://rdf.myexperiment.org/ontologies/base/",
  content: include_str!("../prefixes/meb.ttl"),
  name: "meb",
  title: "The myExperiment Base Ontology",
}, LocalPrefix {
  location: "http://purl.org/media",
  content: include_str!("../prefixes/media.ttl"),
  name: "media",
  title: "The Media RDF Vocabulary",
}, LocalPrefix {
  location: "http://w3id.org/medred/medred#",
  content: include_str!("../prefixes/medred.ttl"),
  name: "medred",
  title: "MedRed ontology: clinical data acquisition model",
}, LocalPrefix {
  location: "http://mex.aksw.org/mex-algo",
  content: include_str!("../prefixes/mexalgo.ttl"),
  name: "mexalgo",
  title: "MEX Algorithm Ontology",
}, LocalPrefix {
  location: "http://mex.aksw.org/mex-core",
  content: include_str!("../prefixes/mexcore.ttl"),
  name: "mexcore",
  title: "MEX Core Vocabulary",
}, LocalPrefix {
  location: "http://mex.aksw.org/mex-perf",
  content: include_str!("../prefixes/mexperf.ttl"),
  name: "mexperf",
  title: "MEX Performance Ontology",
}, LocalPrefix {
  location: "http://rdf.muninn-project.org/ontologies/military",
  content: include_str!("../prefixes/mil.ttl"),
  name: "mil",
  title: "Military Ontology Specification",
}, LocalPrefix {
  location: "http://www.w3.org/ns/mls",
  content: include_str!("../prefixes/mls.ttl"),
  name: "mls",
  title: "Machine Learning Schema",
}, LocalPrefix {
  location: "http://purl.org/ontology/mo/",
  content: include_str!("../prefixes/mo.ttl"),
  name: "mo",
  title: "Music Ontology",
}, LocalPrefix {
  location: "http://www.observedchange.com/moac/ns#",
  content: include_str!("../prefixes/moac.ttl"),
  name: "moac",
  title: "Management of a Crisis Vocabulary",
}, LocalPrefix {
  location: "http://moat-project.org/ns#",
  content: include_str!("../prefixes/moat.ttl"),
  name: "moat",
  title: "Meaning of a Tag Ontology",
}, LocalPrefix {
  location: "http://www.isibang.ac.in/ns/mod",
  content: include_str!("../prefixes/mod.ttl"),
  name: "mod",
  title: "MOD: Metadata for Ontology Description and publication",
}, LocalPrefix {
  location: "https://w3id.org/mod",
  content: include_str!("../prefixes/modp.ttl"),
  name: "modp",
  title: "Metadata for Ontology Description and publication",
}, LocalPrefix {
  location: "https://w3id.org/skgo/modsci#",
  content: include_str!("../prefixes/modsci.ttl"),
  name: "modsci",
  title: "ModSci, Modern Science Ontology.",
}, LocalPrefix {
  location: "http://id.loc.gov/vocabulary/relators",
  content: include_str!("../prefixes/mrel.ttl"),
  name: "mrel",
  title: "MARC Code List for Relators",
}, LocalPrefix {
  location: "http://iserve.kmi.open.ac.uk/ns/msm",
  content: include_str!("../prefixes/msm.ttl"),
  name: "msm",
  title: "Minimal Service Model",
}, LocalPrefix {
  location: "https://www.purl.org/mso-em",
  content: include_str!("../prefixes/mso-em.ttl"),
  name: "mso-em",
  title: "MSO-EM: Ontologies for modelling, simulation, optimization (MSO) and epistemic metadata (EM)",
}, LocalPrefix {
  location: "http://www.telegraphis.net/ontology/measurement/measurement#",
  content: include_str!("../prefixes/msr.ttl"),
  name: "msr",
  title: "Measurement Ontology",
}, LocalPrefix {
  location: "http://www.ics.forth.gr/isl/MarineTLO/v4/marinetlo.owl",
  content: include_str!("../prefixes/mtlo.ttl"),
  name: "mtlo",
  title: "MarineTLO Ontology",
}, LocalPrefix {
  location: "http://ns.inria.fr/munc/",
  content: include_str!("../prefixes/munc.ttl"),
  name: "munc",
  title: "Meta-Uncertainty",
}, LocalPrefix {
  location: "http://data.doremus.org/ontology#",
  content: include_str!("../prefixes/mus.ttl"),
  name: "mus",
  title: "DOREMUS is an extension of the FRBRoo model for describing the music.",
}, LocalPrefix {
  location: "http://www.kanzaki.com/ns/music",
  content: include_str!("../prefixes/music.ttl"),
  name: "music",
  title: "Music Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/muto/core",
  content: include_str!("../prefixes/muto.ttl"),
  name: "muto",
  title: "Modular Unified Tagging Ontology (MUTO)",
}, LocalPrefix {
  location: "http://schema.mobivoc.org/",
  content: include_str!("../prefixes/mv.ttl"),
  name: "mv",
  title: "MobiVoc: Open Mobility Vocabulary",
}, LocalPrefix {
  location: "http://purl.oclc.org/NET/mvco.owl",
  content: include_str!("../prefixes/mvco.ttl"),
  name: "mvco",
  title: "Media Value Chain Ontology",
}, LocalPrefix {
  location: "http://www.semanticdesktop.org/ontologies/2007/08/15/nao",
  content: include_str!("../prefixes/nao.ttl"),
  name: "nao",
  title: "NEPOMUK Annotation Ontology",
}, LocalPrefix {
  location: "https://data.nasa.gov/ontologies/atmonto/NAS#",
  content: include_str!("../prefixes/nas.ttl"),
  name: "nas",
  title: "US National Airspace System (NAS) vocabulary",
}, LocalPrefix {
  location: "http://www.semanticdesktop.org/ontologies/2007/04/02/ncal",
  content: include_str!("../prefixes/ncal.ttl"),
  name: "ncal",
  title: "NEPOMUK Calendar Ontology",
}, LocalPrefix {
  location: "http://www.semanticdesktop.org/ontologies/2007/03/22/nco",
  content: include_str!("../prefixes/nco.ttl"),
  name: "nco",
  title: "NEPOMUK Contact Ontology",
}, LocalPrefix {
  location: "https://w3id.org/nen2660/",
  content: include_str!("../prefixes/nen2660.ttl"),
  name: "nen2660",
  title: "NEN 2660-2:2022 'Rules for information modelling of the built environment - Part 2: Practical configuration, extension and implementation of NEN 2660-1''",
}, LocalPrefix {
  location: "http://modellen.geostandaarden.nl/def/nen3610",
  content: include_str!("../prefixes/nen3610.ttl"),
  name: "nen3610",
  title: "NEN 3610 - base model geo-information",
}, LocalPrefix {
  location: "http://www.semanticdesktop.org/ontologies/2007/03/22/nfo",
  content: include_str!("../prefixes/nfo.ttl"),
  name: "nfo",
  title: "NEPOMUK File Ontology",
}, LocalPrefix {
  location: "http://geovocab.org/geometry",
  content: include_str!("../prefixes/ngeo.ttl"),
  name: "ngeo",
  title: "NeoGeo Geometry Ontology",
}, LocalPrefix {
  location: "http://www.semanticdesktop.org/ontologies/2007/01/19/nie",
  content: include_str!("../prefixes/nie.ttl"),
  name: "nie",
  title: "NEPOMUK Information Element Core Ontology",
}, LocalPrefix {
  location: "http://persistence.uni-leipzig.org/nlp2rdf/ontologies/nif-core#",
  content: include_str!("../prefixes/nif.ttl"),
  name: "nif",
  title: "NLP Interchange Format",
}, LocalPrefix {
  location: "http://lod.nl.go.kr/ontology/",
  content: include_str!("../prefixes/nlon.ttl"),
  name: "nlon",
  title: "National Library of Korea Ontology",
}, LocalPrefix {
  location: "https://w3id.org/nno/ontology",
  content: include_str!("../prefixes/nno.ttl"),
  name: "nno",
  title: "The Neural Network Ontology",
}, LocalPrefix {
  location: "https://w3id.org/noria/ontology/",
  content: include_str!("../prefixes/noria.ttl"),
  name: "noria",
  title: "The NORIA Ontology",
}, LocalPrefix {
  location: "http://www.nanopub.org/nschema",
  content: include_str!("../prefixes/np.ttl"),
  name: "np",
  title: "Nano publication ontology",
}, LocalPrefix {
  location: "http://ns.nature.com/terms/",
  content: include_str!("../prefixes/npg.ttl"),
  name: "npg",
  title: "Nature.com Core Ontology",
}, LocalPrefix {
  location: "http://www.semanticdesktop.org/ontologies/2007/08/15/nrl",
  content: include_str!("../prefixes/nrl.ttl"),
  name: "nrl",
  title: "NEPOMUK Representational Language",
}, LocalPrefix {
  location: "http://ns.inria.fr/nrv",
  content: include_str!("../prefixes/nrv.ttl"),
  name: "nrv",
  title: "Normative Requirements Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/ontology/storyline",
  content: include_str!("../prefixes/nsl.ttl"),
  name: "nsl",
  title: "A News Storyline Ontology",
}, LocalPrefix {
  location: "http://ns.inria.fr/nicetag/2010/09/09/voc",
  content: include_str!("../prefixes/ntag.ttl"),
  name: "ntag",
  title: "Nice Tag Ontology",
}, LocalPrefix {
  location: "https://w3id.org/def/nyon#",
  content: include_str!("../prefixes/nyon.ttl"),
  name: "nyon",
  title: "NyOn: A Multilingual Legal Ontology for Globalized Judicial System",
}, LocalPrefix {
  location: "http://www.w3.org/ns/oa#",
  content: include_str!("../prefixes/oa.ttl"),
  name: "oa",
  title: "Open Annotation Data Model",
}, LocalPrefix {
  location: "http://culturalis.org/oad#",
  content: include_str!("../prefixes/oad.ttl"),
  name: "oad",
  title: "Ontology for archival description",
}, LocalPrefix {
  location: "http://www.ics.forth.gr/isl/oae/core",
  content: include_str!("../prefixes/oae.ttl"),
  name: "oae",
  title: "Open NEE Model",
}, LocalPrefix {
  location: "http://data.lirmm.fr/ontologies/oan",
  content: include_str!("../prefixes/oan.ttl"),
  name: "oan",
  title: "Ontology of the French National Assembly",
}, LocalPrefix {
  location: "http://purl.obolibrary.org/obo/obi.owl",
  content: include_str!("../prefixes/obo.ttl"),
  name: "obo",
  title: "Ontology for Biomedical Investigation",
}, LocalPrefix {
  location: "http://rdf.geospecies.org/methods/observationMethod.rdf",
  content: include_str!("../prefixes/obsm.ttl"),
  name: "obsm",
  title: "Observation Method Ontology",
}, LocalPrefix {
  location: "http://delicias.dia.fi.upm.es/ontologies/ObjectWithStates.owl",
  content: include_str!("../prefixes/obws.ttl"),
  name: "obws",
  title: "Object with states ontology",
}, LocalPrefix {
  location: "http://contextus.net/ontology/ontomedia/core/expression#",
  content: include_str!("../prefixes/oc.ttl"),
  name: "oc",
  title: "OntoMedia Core",
}, LocalPrefix {
  location: "http://dati.camera.it/ocd/",
  content: include_str!("../prefixes/ocd.ttl"),
  name: "ocd",
  title: "Ontology of Italian Deputy Chamber",
}, LocalPrefix {
  location: "http://purl.org/onto-ocds/ocds",
  content: include_str!("../prefixes/ocds.ttl"),
  name: "ocds",
  title: "Schema for an Open Contracting Release (OCDS)",
}, LocalPrefix {
  location: "https://w3id.org/ontouml-models/vocabulary",
  content: include_str!("../prefixes/ocmv.ttl"),
  name: "ocmv",
  title: "OntoUML/UFO Catalog Metadata Vocabulary",
}, LocalPrefix {
  location: "http://vocab.deri.ie/odapp",
  content: include_str!("../prefixes/odapp.ttl"),
  name: "odapp",
  title: "Open Data Applications Vocabulary",
}, LocalPrefix {
  location: "http://semweb.mmlab.be/ns/odapps",
  content: include_str!("../prefixes/odapps.ttl"),
  name: "odapps",
  title: "The vocabulary for (L)OD ideas and applications",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/cp/owl/participation.owl",
  content: include_str!("../prefixes/odpart.ttl"),
  name: "odpart",
  title: "Ontology Design Pattern Participation",
}, LocalPrefix {
  location: "http://www.w3.org/ns/odrl/2/",
  content: include_str!("../prefixes/odrl.ttl"),
  name: "odrl",
  title: "The Open Digital Rights Language (ODRL) Ontology",
}, LocalPrefix {
  location: "http://schema.theodi.org/odrs",
  content: include_str!("../prefixes/odrs.ttl"),
  name: "odrs",
  title: "Open Data Rights Statement Vocabulary",
}, LocalPrefix {
  location: "http://reference.data.gov.uk/def/organogram",
  content: include_str!("../prefixes/odv.ttl"),
  name: "odv",
  title: "Organogram Data Vocabulary",
}, LocalPrefix {
  location: "http://www.oegov.org/core/owl/cc",
  content: include_str!("../prefixes/oecc.ttl"),
  name: "oecc",
  title: "Extended Creative Commons Ontology",
}, LocalPrefix {
  location: "http://owlrep.eu01.aws.af.cm/fridge",
  content: include_str!("../prefixes/of.ttl"),
  name: "of",
  title: "Open Fridge vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/ofo#",
  content: include_str!("../prefixes/ofo.ttl"),
  name: "ofo",
  title: "Occupant Feedback Ontology",
}, LocalPrefix {
  location: "http://purl.org/opdm/refrigerator#",
  content: include_str!("../prefixes/ofrd.ttl"),
  name: "ofrd",
  title: "Fridge and Freezer Vocabulary",
}, LocalPrefix {
  location: "http://ogp.me/ns",
  content: include_str!("../prefixes/og.ttl"),
  name: "og",
  title: "Open Graph Protocol Vocabulary",
}, LocalPrefix {
  location: "http://semweb.mmlab.be/ns/oh",
  content: include_str!("../prefixes/oh.ttl"),
  name: "oh",
  title: "The Opening Hours vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/oseg/ont/okh",
  content: include_str!("../prefixes/okh.ttl"),
  name: "okh",
  title: "Open Know How (OKH) ontology",
}, LocalPrefix {
  location: "https://w3id.org/vocab/olca",
  content: include_str!("../prefixes/olca.ttl"),
  name: "olca",
  title: "Ontology Loose Coupling Annotation",
}, LocalPrefix {
  location: "http://purl.org/ontology/olo/core#",
  content: include_str!("../prefixes/olo.ttl"),
  name: "olo",
  title: "Ordered List Ontology",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19156/2011/observation",
  content: include_str!("../prefixes/om.ttl"),
  name: "om",
  title: "ISO 19156 Observation Model",
}, LocalPrefix {
  location: "https://w3id.org/omg",
  content: include_str!("../prefixes/omg.ttl"),
  name: "omg",
  title: "OMG: Ontology for Managing Geometry",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/ontology/om/om-lite",
  content: include_str!("../prefixes/oml.ttl"),
  name: "oml",
  title: "OWL for Observations",
}, LocalPrefix {
  location: "http://open-multinet.info/ontology/omn",
  content: include_str!("../prefixes/omn.ttl"),
  name: "omn",
  title: "Open-Multinet Upper Ontology",
}, LocalPrefix {
  location: "http://open-multinet.info/ontology/omn-federation",
  content: include_str!("../prefixes/omnfed.ttl"),
  name: "omnfed",
  title: "Open-Multinet Upper Federation Ontology",
}, LocalPrefix {
  location: "http://open-multinet.info/ontology/omn-lifecycle",
  content: include_str!("../prefixes/omnlc.ttl"),
  name: "omnlc",
  title: "Open-Multinet Upper Lifecycle Ontology",
}, LocalPrefix {
  location: "http://www.ics.forth.gr/isl/oncm/core",
  content: include_str!("../prefixes/onc.ttl"),
  name: "onc",
  title: "Open NEE Configuration Model",
}, LocalPrefix {
  location: "http://purl.org/net/ns/ontology-annot",
  content: include_str!("../prefixes/ont.ttl"),
  name: "ont",
  title: "Ontology annotation DLiser vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/lemon/ontolex",
  content: include_str!("../prefixes/ontolex.ttl"),
  name: "ontolex",
  title: "Lexicon Model for Ontologies - Core",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/ont/dul/ontopic.owl",
  content: include_str!("../prefixes/ontopic.ttl"),
  name: "ontopic",
  title: "Ontopic Ontology",
}, LocalPrefix {
  location: "http://www.semanticweb.org/ontologies/2008/11/OntologySecurity.owl",
  content: include_str!("../prefixes/ontosec.ttl"),
  name: "ontosec",
  title: "Ontology Security",
}, LocalPrefix {
  location: "https://w3id.org/ontouml",
  content: include_str!("../prefixes/ontouml.ttl"),
  name: "ontouml",
  title: "OntoUML Metamodel Vocabulary",
}, LocalPrefix {
  location: "http://www.gsi.dit.upm.es/ontologies/onyx/ns",
  content: include_str!("../prefixes/onyx.ttl"),
  name: "onyx",
  title: "Onyx Emotion Ontology",
}, LocalPrefix {
  location: "http://purl.org/openorg/",
  content: include_str!("../prefixes/oo.ttl"),
  name: "oo",
  title: "Open Organisations",
}, LocalPrefix {
  location: "http://environment.data.gov.au/def/op",
  content: include_str!("../prefixes/op.ttl"),
  name: "op",
  title: "Observable properties",
}, LocalPrefix {
  location: "http://openprovenance.org/model/opmo",
  content: include_str!("../prefixes/opmo.ttl"),
  name: "opmo",
  title: "Open Provenance Model",
}, LocalPrefix {
  location: "http://purl.org/net/opmv/ns#",
  content: include_str!("../prefixes/opmv.ttl"),
  name: "opmv",
  title: "Open Provenance Model Vocabulary",
}, LocalPrefix {
  location: "http://www.opmw.org/ontology/",
  content: include_str!("../prefixes/opmw.ttl"),
  name: "opmw",
  title: "The OPMW Ontology",
}, LocalPrefix {
  location: "http://online-presence.net/opo/ns#",
  content: include_str!("../prefixes/opo.ttl"),
  name: "opo",
  title: "Online Presence Ontology",
}, LocalPrefix {
  location: "http://lsdis.cs.uga.edu/projects/semdis/opus#",
  content: include_str!("../prefixes/opus.ttl"),
  name: "opus",
  title: "SwetoDblp Ontology of Computer Science Publications",
}, LocalPrefix {
  location: "http://vocab.deri.ie/orca",
  content: include_str!("../prefixes/orca.ttl"),
  name: "orca",
  title: "orca, the Ontology of Reasoning, Certainty and Attribution",
}, LocalPrefix {
  location: "http://www.openarchives.org/ore/terms/",
  content: include_str!("../prefixes/ore.ttl"),
  name: "ore",
  title: "The OAI ORE terms vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/org#",
  content: include_str!("../prefixes/org.ttl"),
  name: "org",
  title: "Core organization ontology",
}, LocalPrefix {
  location: "http://datos.gob.es/def/sector-publico/organizacion#",
  content: include_str!("../prefixes/orges.ttl"),
  name: "orges",
  title: "Ontology about Spanish public organizations",
}, LocalPrefix {
  location: "http://data.ordnancesurvey.co.uk/ontology/admingeo/",
  content: include_str!("../prefixes/osadm.ttl"),
  name: "osadm",
  title: "The administrative geography and civil voting area ontology",
}, LocalPrefix {
  location: "http://data.ordnancesurvey.co.uk/ontology/geometry/",
  content: include_str!("../prefixes/osgeom.ttl"),
  name: "osgeom",
  title: "Ordnance Survey Geometry Ontology",
}, LocalPrefix {
  location: "http://open-services.net/ns/core#",
  content: include_str!("../prefixes/oslc.ttl"),
  name: "oslc",
  title: "OSLC Core Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/oslo/ns/localgov",
  content: include_str!("../prefixes/oslo.ttl"),
  name: "oslo",
  title: "OSLO Exchange Standard",
}, LocalPrefix {
  location: "http://data.lirmm.fr/ontologies/osp",
  content: include_str!("../prefixes/osp.ttl"),
  name: "osp",
  title: "French Public Services Ontology",
}, LocalPrefix {
  location: "http://contextus.net/ontology/ontomedia/core/space#",
  content: include_str!("../prefixes/osr.ttl"),
  name: "osr",
  title: "OntoMedia Space Representation",
}, LocalPrefix {
  location: "http://data.ordnancesurvey.co.uk/ontology/spatialrelations/",
  content: include_str!("../prefixes/osspr.ttl"),
  name: "osspr",
  title: "Spatial Relations Ontology",
}, LocalPrefix {
  location: "http://www.ordnancesurvey.co.uk/ontology/Topography/v0.1/Topography.owl",
  content: include_str!("../prefixes/ostop.ttl"),
  name: "ostop",
  title: "Ordnance Survey Topography Ontology",
}, LocalPrefix {
  location: "https://w3id.org/opentrafficlights",
  content: include_str!("../prefixes/otl.ttl"),
  name: "otl",
  title: "The Open Traffic Lights ontology",
}, LocalPrefix {
  location: "http://www.ontology-of-units-of-measure.org/resource/om-2/",
  content: include_str!("../prefixes/oum.ttl"),
  name: "oum",
  title: "Ontology of units of Measure (OM)",
}, LocalPrefix {
  location: "http://open.vocab.org/terms",
  content: include_str!("../prefixes/ov.ttl"),
  name: "ov",
  title: "OpenVocab",
}, LocalPrefix {
  location: "http://www.w3.org/2002/07/owl",
  content: include_str!("../prefixes/owl.ttl"),
  name: "owl",
  title: "The OWL 2 Schema vocabulary",
}, LocalPrefix {
  location: "http://purl.org/net/p-plan#",
  content: include_str!("../prefixes/p-plan.ttl"),
  name: "p-plan",
  title: "The P-PLAN Ontology",
}, LocalPrefix {
  location: "http://reference.data.gov.uk/def/parliament",
  content: include_str!("../prefixes/parl.ttl"),
  name: "parl",
  title: "Parliament Ontology",
}, LocalPrefix {
  location: "http://purl.org/vocab/participation/schema",
  content: include_str!("../prefixes/part.ttl"),
  name: "part",
  title: "Participation Schema",
}, LocalPrefix {
  location: "http://data.lirmm.fr/ontologies/passim",
  content: include_str!("../prefixes/passim.ttl"),
  name: "passim",
  title: "PASSIM ontology",
}, LocalPrefix {
  location: "http://purl.org/hpi/patchr#",
  content: include_str!("../prefixes/pat.ttl"),
  name: "pat",
  title: "Patch Request Ontology",
}, LocalPrefix {
  location: "http://www.essepuntato.it/2008/12/pattern",
  content: include_str!("../prefixes/pattern.ttl"),
  name: "pattern",
  title: "The Pattern Ontology",
}, LocalPrefix {
  location: "http://purl.org/pav/",
  content: include_str!("../prefixes/pav.ttl"),
  name: "pav",
  title: "Provenance, Authoring and Versioning",
}, LocalPrefix {
  location: "http://reference.data.gov.uk/def/payment#",
  content: include_str!("../prefixes/pay.ttl"),
  name: "pay",
  title: "Payments ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/pbo/core#",
  content: include_str!("../prefixes/pbo.ttl"),
  name: "pbo",
  title: "Play Back Ontology",
}, LocalPrefix {
  location: "http://purl.org/procurement/public-contracts",
  content: include_str!("../prefixes/pc.ttl"),
  name: "pc",
  title: "Public Contracts Ontology",
}, LocalPrefix {
  location: "https://w3id.org/dpv/pd",
  content: include_str!("../prefixes/pd.ttl"),
  name: "pd",
  title: "Personal Data Categories",
}, LocalPrefix {
  location: "http://vocab.deri.ie/pdo",
  content: include_str!("../prefixes/pdo.ttl"),
  name: "pdo",
  title: "Project Documents Ontology",
}, LocalPrefix {
  location: "https://w3id.org/peco",
  content: include_str!("../prefixes/peco.ttl"),
  name: "peco",
  title: "The Provenance of Emission Calculations Ontology",
}, LocalPrefix {
  location: "https://w3id.org/pep/",
  content: include_str!("../prefixes/pep.ttl"),
  name: "pep",
  title: "Process Execution ontology.",
}, LocalPrefix {
  location: "http://www.w3.org/ns/person",
  content: include_str!("../prefixes/person.ttl"),
  name: "person",
  title: "ISA Programme Person Core Vocabulary",
}, LocalPrefix {
  location: "http://www.ontotext.com/proton/protonext",
  content: include_str!("../prefixes/pext.ttl"),
  name: "pext",
  title: "PROTON Extent module",
}, LocalPrefix {
  location: "http://rdf-vocabulary.ddialliance.org/phdd",
  content: include_str!("../prefixes/phdd.ttl"),
  name: "phdd",
  title: "Physical Data Description",
}, LocalPrefix {
  location: "http://data.cochrane.org/ontologies/pico/",
  content: include_str!("../prefixes/pico.ttl"),
  name: "pico",
  title: "Cochrane PICO Ontology",
}, LocalPrefix {
  location: "http://www.molmod.info/semantics/pims-ii.ttl",
  content: include_str!("../prefixes/pimsii.ttl"),
  name: "pimsii",
  title: "Physicalistic Interpretation of Modelling and Simulation - Interoperability Infrastructure (PIMS-II)",
}, LocalPrefix {
  location: "http://purl.org/ontology/places",
  content: include_str!("../prefixes/place.ttl"),
  name: "place",
  title: "The Places Ontology",
}, LocalPrefix {
  location: "http://cedric.cnam.fr/isid/ontologies/PersonLink.owl",
  content: include_str!("../prefixes/plink.ttl"),
  name: "plink",
  title: "PersonLink Ontology",
}, LocalPrefix {
  location: "http://purl.org/net/po#",
  content: include_str!("../prefixes/plo.ttl"),
  name: "plo",
  title: "Playlist Ontology",
}, LocalPrefix {
  location: "http://inference-web.org/2.0/pml-provenance.owl",
  content: include_str!("../prefixes/pmlp.ttl"),
  name: "pmlp",
  title: "PML2 provenance ontology",
}, LocalPrefix {
  location: "http://premon.fbk.eu/ontology/fn",
  content: include_str!("../prefixes/pmofn.ttl"),
  name: "pmofn",
  title: "Predicate Model for Ontologies (PreMOn) - FrameNet ontology module",
}, LocalPrefix {
  location: "http://premon.fbk.eu/ontology/nb",
  content: include_str!("../prefixes/pmonb.ttl"),
  name: "pmonb",
  title: "Predicate Model for Ontologies (PreMOn) - NomBank ontology module",
}, LocalPrefix {
  location: "http://premon.fbk.eu/ontology/pb",
  content: include_str!("../prefixes/pmopb.ttl"),
  name: "pmopb",
  title: "Predicate Model for Ontologies (PreMOn) - PropBank ontology module",
}, LocalPrefix {
  location: "http://premon.fbk.eu/ontology/vn",
  content: include_str!("../prefixes/pmovn.ttl"),
  name: "pmovn",
  title: "Predicate Model for Ontologies (PreMOn) - VerbNet ontology module",
}, LocalPrefix {
  location: "http://data.press.net/ontology/asset/",
  content: include_str!("../prefixes/pna.ttl"),
  name: "pna",
  title: "Press.net Asset Ontology",
}, LocalPrefix {
  location: "http://data.press.net/ontology/classification/",
  content: include_str!("../prefixes/pnc.ttl"),
  name: "pnc",
  title: "Press.net Classification Ontology",
}, LocalPrefix {
  location: "http://data.press.net/ontology/event/",
  content: include_str!("../prefixes/pne.ttl"),
  name: "pne",
  title: "Press.net Event Ontology",
}, LocalPrefix {
  location: "http://data.press.net/ontology/identifier/",
  content: include_str!("../prefixes/pni.ttl"),
  name: "pni",
  title: "SNaP Identifier Ontology",
}, LocalPrefix {
  location: "http://data.press.net/ontology/stuff/",
  content: include_str!("../prefixes/pns.ttl"),
  name: "pns",
  title: "Press.net Stuff Ontology",
}, LocalPrefix {
  location: "http://data.press.net/ontology/tag/",
  content: include_str!("../prefixes/pnt.ttl"),
  name: "pnt",
  title: "Press.net Tag Ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/po/",
  content: include_str!("../prefixes/po.ttl"),
  name: "po",
  title: "Programmes ontology",
}, LocalPrefix {
  location: "http://dev.poderopedia.com/vocab/schema",
  content: include_str!("../prefixes/poder.ttl"),
  name: "poder",
  title: "Poder Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/poso/",
  content: include_str!("../prefixes/poso.ttl"),
  name: "poso",
  title: "Positioning System Ontology",
}, LocalPrefix {
  location: "http://purl.org/poso/common/",
  content: include_str!("../prefixes/posocm.ttl"),
  name: "posocm",
  title: "Positioning System Ontology Common Technologies, Algorithms and Systems",
}, LocalPrefix {
  location: "http://data.ordnancesurvey.co.uk/ontology/postcode/",
  content: include_str!("../prefixes/postcode.ttl"),
  name: "postcode",
  title: "Postcode Ontology",
}, LocalPrefix {
  location: "http://data.lirmm.fr/ontologies/poste",
  content: include_str!("../prefixes/poste.ttl"),
  name: "poste",
  title: "\"La Poste\" Ontology",
}, LocalPrefix {
  location: "http://vocab.deri.ie/ppo",
  content: include_str!("../prefixes/ppo.ttl"),
  name: "ppo",
  title: "Privacy Preference Ontology",
}, LocalPrefix {
  location: "http://contsem.unizar.es/def/sector-publico/pproc",
  content: include_str!("../prefixes/pproc.ttl"),
  name: "pproc",
  title: "PPROC ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/prv/core#",
  content: include_str!("../prefixes/pr.ttl"),
  name: "pr",
  title: "Property Reification Vocabulary",
}, LocalPrefix {
  location: "http://www.loc.gov/premis/rdf/v1",
  content: include_str!("../prefixes/premis.ttl"),
  name: "premis",
  title: "PREMIS Ontology",
}, LocalPrefix {
  location: "http://ns.inria.fr/prissma/v2#",
  content: include_str!("../prefixes/prissma.ttl"),
  name: "prissma",
  title: "Presentation of Resources for Interoperable Semantic and Shareable Mobile Adaptability",
}, LocalPrefix {
  location: "http://purl.org/spar/pro",
  content: include_str!("../prefixes/pro.ttl"),
  name: "pro",
  title: "The Publishing Roles Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/ns/dx/prof",
  content: include_str!("../prefixes/prof.ttl"),
  name: "prof",
  title: "Profiles Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/prog/",
  content: include_str!("../prefixes/prog.ttl"),
  name: "prog",
  title: "The Event Programme Vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/prov#",
  content: include_str!("../prefixes/prov.ttl"),
  name: "prov",
  title: "W3C PROVenance Interchange",
}, LocalPrefix {
  location: "http://ns.inria.fr/provoc",
  content: include_str!("../prefixes/provoc.ttl"),
  name: "provoc",
  title: "Product Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/net/provenance/ns#",
  content: include_str!("../prefixes/prv.ttl"),
  name: "prv",
  title: "Provenance Vocabulary Core Ontology",
}, LocalPrefix {
  location: "http://purl.org/net/provenance/types#",
  content: include_str!("../prefixes/prvt.ttl"),
  name: "prvt",
  title: "Provenance Vocabulary types",
}, LocalPrefix {
  location: "http://ns.inria.fr/probabilistic-shacl/",
  content: include_str!("../prefixes/psh.ttl"),
  name: "psh",
  title: "Probabilistic SHACL Validation",
}, LocalPrefix {
  location: "https://purl.org/psn/vocab#",
  content: include_str!("../prefixes/psn.ttl"),
  name: "psn",
  title: "Product Supply Network Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/spar/pso",
  content: include_str!("../prefixes/pso.ttl"),
  name: "pso",
  title: "The Publishing Status Ontology",
}, LocalPrefix {
  location: "http://www.ontotext.com/proton/protontop",
  content: include_str!("../prefixes/ptop.ttl"),
  name: "ptop",
  title: "PROTON (Proto Ontology), Top Module",
}, LocalPrefix {
  location: "https://vocab.eccenca.com/pubsub/",
  content: include_str!("../prefixes/pubsub.ttl"),
  name: "pubsub",
  title: "Eccenca Publish-Subscribe Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/env/puv",
  content: include_str!("../prefixes/puv.ttl"),
  name: "puv",
  title: "Parameter Usage Vocabulary ontology",
}, LocalPrefix {
  location: "http://purl.org/spar/pwo",
  content: include_str!("../prefixes/pwo.ttl"),
  name: "pwo",
  title: "The Publishing Workflow Ontology",
}, LocalPrefix {
  location: "http://purl.org/linked-data/cube",
  content: include_str!("../prefixes/qb.ttl"),
  name: "qb",
  title: "The data cube vocabulary",
}, LocalPrefix {
  location: "http://purl.org/qb4olap/cubes",
  content: include_str!("../prefixes/qb4o.ttl"),
  name: "qb4o",
  title: "Vocabulary for publishing OLAP data cubes",
}, LocalPrefix {
  location: "http://purl.oclc.org/NET/ssnx/qu/qu",
  content: include_str!("../prefixes/qu.ttl"),
  name: "qu",
  title: "Quantity Kinds and Units",
}, LocalPrefix {
  location: "http://qudt.org/schema/qudt",
  content: include_str!("../prefixes/qudt.ttl"),
  name: "qudt",
  title: "Quantities, Units, Dimensions and Types",
}, LocalPrefix {
  location: "https://w3id.org/arco/ontology/arco",
  content: include_str!("../prefixes/r-arco.ttl"),
  name: "r-arco",
  title: "ArCo Ontology (ArCo network)",
}, LocalPrefix {
  location: "http://guava.iis.sinica.edu.tw/r4r",
  content: include_str!("../prefixes/r4r.ttl"),
  name: "r4r",
  title: "Relations for Reusing (R4R) Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/ns/radion#",
  content: include_str!("../prefixes/radion.ttl"),
  name: "radion",
  title: "Repository Asset Distribution",
}, LocalPrefix {
  location: "https://w3id.org/rains",
  content: include_str!("../prefixes/rains.ttl"),
  name: "rains",
  title: "The RAInS Ontology",
}, LocalPrefix {
  location: "https://w3id.org/i40/rami/",
  content: include_str!("../prefixes/rami.ttl"),
  name: "rami",
  title: "rami - Reference Architecture Model",
}, LocalPrefix {
  location: "http://vocab.deri.ie/raul",
  content: include_str!("../prefixes/raul.ttl"),
  name: "raul",
  title: "RAUL Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/riverbench/schema/metadata",
  content: include_str!("../prefixes/rb.ttl"),
  name: "rb",
  title: "RiverBench metadata ontology",
}, LocalPrefix {
  location: "https://w3id.org/riverbench/schema/documentation",
  content: include_str!("../prefixes/rbdoc.ttl"),
  name: "rbdoc",
  title: "RiverBench documentation ontology",
}, LocalPrefix {
  location: "http://rdaregistry.info/Elements/a",
  content: include_str!("../prefixes/rdaa.ttl"),
  name: "rdaa",
  title: "RDA Agent properties",
}, LocalPrefix {
  location: "http://rdaregistry.info/Elements/c",
  content: include_str!("../prefixes/rdac.ttl"),
  name: "rdac",
  title: "RDA Classes",
}, LocalPrefix {
  location: "http://rdaregistry.info/Elements/e",
  content: include_str!("../prefixes/rdae.ttl"),
  name: "rdae",
  title: "RDA Expression properties",
}, LocalPrefix {
  location: "http://rdvocab.info/uri/schema/FRBRentitiesRDA",
  content: include_str!("../prefixes/rdafrbr.ttl"),
  name: "rdafrbr",
  title: "FRBR Entities for RDA",
}, LocalPrefix {
  location: "http://rdvocab.info/Elements",
  content: include_str!("../prefixes/rdag1.ttl"),
  name: "rdag1",
  title: "RDA Group 1 Elements",
}, LocalPrefix {
  location: "http://rdvocab.info/ElementsGr2",
  content: include_str!("../prefixes/rdag2.ttl"),
  name: "rdag2",
  title: "RDA Group 2 Elements",
}, LocalPrefix {
  location: "http://rdvocab.info/ElementsGr3",
  content: include_str!("../prefixes/rdag3.ttl"),
  name: "rdag3",
  title: "RDA Group 3 Elements",
}, LocalPrefix {
  location: "http://rdaregistry.info/Elements/i",
  content: include_str!("../prefixes/rdai.ttl"),
  name: "rdai",
  title: "RDA Item properties",
}, LocalPrefix {
  location: "http://rdaregistry.info/Elements/m",
  content: include_str!("../prefixes/rdam.ttl"),
  name: "rdam",
  title: "RDA Manifestation properties",
}, LocalPrefix {
  location: "http://rdvocab.info/RDARelationshipsWEMI",
  content: include_str!("../prefixes/rdarel.ttl"),
  name: "rdarel",
  title: "RDA Relationships for Works, Expressions, Manifestations, Items",
}, LocalPrefix {
  location: "http://metadataregistry.org/uri/schema/RDARelationshipsGR2",
  content: include_str!("../prefixes/rdarel2.ttl"),
  name: "rdarel2",
  title: "RDA Relationships GR2",
}, LocalPrefix {
  location: "http://rdvocab.info/roles",
  content: include_str!("../prefixes/rdarole.ttl"),
  name: "rdarole",
  title: "RDA Roles",
}, LocalPrefix {
  location: "http://rdaregistry.info/Elements/u",
  content: include_str!("../prefixes/rdau.ttl"),
  name: "rdau",
  title: "RDA Unconstrained properties",
}, LocalPrefix {
  location: "http://rdaregistry.info/Elements/w",
  content: include_str!("../prefixes/rdaw.ttl"),
  name: "rdaw",
  title: "RDA Work properties",
}, LocalPrefix {
  location: "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
  content: include_str!("../prefixes/rdf.ttl"),
  name: "rdf",
  title: "The RDF Concepts Vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/rdfa#",
  content: include_str!("../prefixes/rdfa.ttl"),
  name: "rdfa",
  title: "RDFa Vocabulary for Term and Prefix Assignment, and for Processor Graph Reporting",
}, LocalPrefix {
  location: "http://www.w3.org/2004/03/trix/rdfg-1/",
  content: include_str!("../prefixes/rdfg.ttl"),
  name: "rdfg",
  title: "Graph",
}, LocalPrefix {
  location: "https://w3id.org/rdfp/",
  content: include_str!("../prefixes/rdfp.ttl"),
  name: "rdfp",
  title: "The RDF Presentation ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2000/01/rdf-schema#",
  content: include_str!("../prefixes/rdfs.ttl"),
  name: "rdfs",
  title: "The RDF Schema vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/rdftest",
  content: include_str!("../prefixes/rdft.ttl"),
  name: "rdft",
  title: "The RDF 1.1 Test Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/react",
  content: include_str!("../prefixes/react.ttl"),
  name: "react",
  title: "The REACT Ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/rec/core#",
  content: include_str!("../prefixes/rec.ttl"),
  name: "rec",
  title: "Recommendation Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2001/02pd/rec54#",
  content: include_str!("../prefixes/rec54.ttl"),
  name: "rec54",
  title: "Model of the W3C Process",
}, LocalPrefix {
  location: "http://purl.org/reco#",
  content: include_str!("../prefixes/reco.ttl"),
  name: "reco",
  title: "RECommendations Ontology",
}, LocalPrefix {
  location: "http://reegle.info/schema",
  content: include_str!("../prefixes/reegle.ttl"),
  name: "reegle",
  title: "Renewable Energy and Energy Efficiency",
}, LocalPrefix {
  location: "http://purl.org/vocab/relationship/",
  content: include_str!("../prefixes/rel.ttl"),
  name: "rel",
  title: "Relationship",
}, LocalPrefix {
  location: "http://www.purl.org/net/remetca#",
  content: include_str!("../prefixes/remetca.ttl"),
  name: "remetca",
  title: "ReMetCa Ontology",
}, LocalPrefix {
  location: "http://purl.org/stuff/rev#",
  content: include_str!("../prefixes/rev.ttl"),
  name: "rev",
  title: "Review Vocabulary",
}, LocalPrefix {
  location: "https://www.ica.org/standards/RiC/ontology",
  content: include_str!("../prefixes/rico.ttl"),
  name: "rico",
  title: "International Council on Archives Records in Contexts Ontology (ICA RiC-O)",
}, LocalPrefix {
  location: "http://persistence.uni-leipzig.org/nlp2rdf/ontologies/rlog#",
  content: include_str!("../prefixes/rlog.ttl"),
  name: "rlog",
  title: "RDF Logging Ontology",
}, LocalPrefix {
  location: "http://w3id.org/rml/core",
  content: include_str!("../prefixes/rml.ttl"),
  name: "rml",
  title: "RML: Generic Mapping Language for RDF",
}, LocalPrefix {
  location: "http://w3id.org/rml/cc/",
  content: include_str!("../prefixes/rml-cc.ttl"),
  name: "rml-cc",
  title: "RML-Containers",
}, LocalPrefix {
  location: "http://w3id.org/rml/fnml/",
  content: include_str!("../prefixes/rml-fnml.ttl"),
  name: "rml-fnml",
  title: "RML-FNML",
}, LocalPrefix {
  location: "http://w3id.org/rml/io/",
  content: include_str!("../prefixes/rml-io.ttl"),
  name: "rml-io",
  title: "RML-IO: Source and Target",
}, LocalPrefix {
  location: "http://w3id.org/rml/star/",
  content: include_str!("../prefixes/rml-star.ttl"),
  name: "rml-star",
  title: "RML-star",
}, LocalPrefix {
  location: "http://purl.org/wf4ever/ro",
  content: include_str!("../prefixes/ro.ttl"),
  name: "ro",
  title: "The Research Object Ontology",
}, LocalPrefix {
  location: "http://w3id.org/roh",
  content: include_str!("../prefixes/roh.ttl"),
  name: "roh",
  title: "The ASIO ontology",
}, LocalPrefix {
  location: "http://vocab.deri.ie/rooms",
  content: include_str!("../prefixes/rooms.ttl"),
  name: "rooms",
  title: "Buildings and Rooms Vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/regorg",
  content: include_str!("../prefixes/rov.ttl"),
  name: "rov",
  title: "Registered Organization Vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/r2rml#",
  content: include_str!("../prefixes/rr.ttl"),
  name: "rr",
  title: "RDB to RDF Mapping Language Schema",
}, LocalPrefix {
  location: "http://softeng.polito.it/rsctx",
  content: include_str!("../prefixes/rsctx.ttl"),
  name: "rsctx",
  title: "Recommender System Context",
}, LocalPrefix {
  location: "http://purl.org/rss/1.0",
  content: include_str!("../prefixes/rss.ttl"),
  name: "rss",
  title: "Vocabulary for Rich Site Summary (RSS) 1.0",
}, LocalPrefix {
  location: "https://w3id.org/rail/topo#",
  content: include_str!("../prefixes/rto.ttl"),
  name: "rto",
  title: "Rail Topology Ontology",
}, LocalPrefix {
  location: "http://purl.org/imbi/ru-meta.owl",
  content: include_str!("../prefixes/ru.ttl"),
  name: "ru",
  title: "Representational Units Metadata Ontology",
}, LocalPrefix {
  location: "http://rdfunit.aksw.org/ns/core#",
  content: include_str!("../prefixes/ruto.ttl"),
  name: "ruto",
  title: "Test-Driven Data Debugging Ontology",
}, LocalPrefix {
  location: "http://ns.inria.fr/s4ac/v2",
  content: include_str!("../prefixes/s4ac.ttl"),
  name: "s4ac",
  title: "Social Semantic SPARQL Security For Access Control Ontology",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4agri/",
  content: include_str!("../prefixes/s4agri.ttl"),
  name: "s4agri",
  title: "SAREF extension for Agriculture",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4bldg/",
  content: include_str!("../prefixes/s4bldg.ttl"),
  name: "s4bldg",
  title: "SAREF extension for building devices",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4city/",
  content: include_str!("../prefixes/s4city.ttl"),
  name: "s4city",
  title: "SAREF extension for Smart City",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4ehaw/",
  content: include_str!("../prefixes/s4ehaw.ttl"),
  name: "s4ehaw",
  title: "SAREF4EHAW: an extension of SAREF for eHealth Ageing Well domain",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4ener/",
  content: include_str!("../prefixes/s4ener.ttl"),
  name: "s4ener",
  title: "SAREF4EE: the EEbus/Energy@home extension of SAREF",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4envi/",
  content: include_str!("../prefixes/s4envi.ttl"),
  name: "s4envi",
  title: "SAREF extension for environment",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4inma/",
  content: include_str!("../prefixes/s4inma.ttl"),
  name: "s4inma",
  title: "SAREF4INMA: an extension of SAREF for the industry and manufacturing domain",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4syst/",
  content: include_str!("../prefixes/s4syst.ttl"),
  name: "s4syst",
  title: "SAREF4SYST: an extension of SAREF for typology of systems and their inter-connections",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4watr/",
  content: include_str!("../prefixes/s4watr.ttl"),
  name: "s4watr",
  title: "SAREF extension for water",
}, LocalPrefix {
  location: "https://saref.etsi.org/saref4wear/",
  content: include_str!("../prefixes/s4wear.ttl"),
  name: "s4wear",
  title: "SAREF4WEAR: an extension of SAREF for Wearables",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19156/2011/sampling",
  content: include_str!("../prefixes/sam.ttl"),
  name: "sam",
  title: "The Sampling Features Schema Vocabulary",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/ontology/om/sam-lite",
  content: include_str!("../prefixes/samfl.ttl"),
  name: "samfl",
  title: "OWL for Sampling Features",
}, LocalPrefix {
  location: "http://dati.san.beniculturali.it/SAN/",
  content: include_str!("../prefixes/san-lod.ttl"),
  name: "san-lod",
  title: "SAN Ontologia",
}, LocalPrefix {
  location: "http://salt.semanticauthoring.org/ontologies/sao",
  content: include_str!("../prefixes/sao.ttl"),
  name: "sao",
  title: "SALT Annotation Ontology",
}, LocalPrefix {
  location: "https://w3id.org/sao",
  content: include_str!("../prefixes/saont.ttl"),
  name: "saont",
  title: "The System Accountability Ontology",
}, LocalPrefix {
  location: "https://saref.etsi.org/core/",
  content: include_str!("../prefixes/saref.ttl"),
  name: "saref",
  title: "SAREF: the Smart Appliances REFerence ontology",
}, LocalPrefix {
  location: "http://purl.org/saws/ontology",
  content: include_str!("../prefixes/saws.ttl"),
  name: "saws",
  title: "Sharing Ancient Wisdoms Ontology",
}, LocalPrefix {
  location: "https://w3id.org/sbeo",
  content: include_str!("../prefixes/sbeo.ttl"),
  name: "sbeo",
  title: "SBEO: Smart Building Evacuation Ontology",
}, LocalPrefix {
  location: "http://schema.org/",
  content: include_str!("../prefixes/schema.ttl"),
  name: "schema",
  title: "Schema.org vocabulary",
}, LocalPrefix {
  location: "http://lod.taxonconcept.org/ontology/sci_people.owl",
  content: include_str!("../prefixes/scip.ttl"),
  name: "scip",
  title: "Scientific People Ontology",
}, LocalPrefix {
  location: "http://purl.org/spar/scoro/",
  content: include_str!("../prefixes/scoro.ttl"),
  name: "scoro",
  title: "Scholarly Contributions and Roles Ontology",
}, LocalPrefix {
  location: "http://rdfs.org/scot/ns#",
  content: include_str!("../prefixes/scot.ttl"),
  name: "scot",
  title: "Social Semantic Cloud of Tags",
}, LocalPrefix {
  location: "http://vocab.deri.ie/scovo",
  content: include_str!("../prefixes/scovo.ttl"),
  name: "scovo",
  title: "The Statistical Core Vocabulary",
}, LocalPrefix {
  location: "http://vocab.deri.ie/scsv",
  content: include_str!("../prefixes/scsv.ttl"),
  name: "scsv",
  title: "Schema.org CSV",
}, LocalPrefix {
  location: "http://www.w3.org/ns/sparql-service-description",
  content: include_str!("../prefixes/sd.ttl"),
  name: "sd",
  title: "SPARQL 1.1 Service Description",
}, LocalPrefix {
  location: "https://w3id.org/vocab/sdm",
  content: include_str!("../prefixes/sdm.ttl"),
  name: "sdm",
  title: "SPARQL endpoint metadata",
}, LocalPrefix {
  location: "http://purl.org/linked-data/sdmx",
  content: include_str!("../prefixes/sdmx.ttl"),
  name: "sdmx",
  title: "SDMX-RDF vocabulary",
}, LocalPrefix {
  location: "http://purl.org/linked-data/sdmx/2009/code",
  content: include_str!("../prefixes/sdmx-code.ttl"),
  name: "sdmx-code",
  title: "SDMX Code",
}, LocalPrefix {
  location: "http://purl.org/linked-data/sdmx/2009/dimension",
  content: include_str!("../prefixes/sdmx-dimension.ttl"),
  name: "sdmx-dimension",
  title: "SDMX Dimension",
}, LocalPrefix {
  location: "http://salt.semanticauthoring.org/ontologies/sdo",
  content: include_str!("../prefixes/sdo.ttl"),
  name: "sdo",
  title: "SALT Document Ontology",
}, LocalPrefix {
  location: "https://w3id.org/okn/o/sdm",
  content: include_str!("../prefixes/sdom.ttl"),
  name: "sdom",
  title: "The Software Description Ontology for Models",
}, LocalPrefix {
  location: "https://w3id.org/okn/o/sd",
  content: include_str!("../prefixes/sdont.ttl"),
  name: "sdont",
  title: "The Software Description Ontology",
}, LocalPrefix {
  location: "http://www.sealitproject.eu/ontology/",
  content: include_str!("../prefixes/sealit.ttl"),
  name: "sealit",
  title: "SeaLiT Ontology",
}, LocalPrefix {
  location: "http://vocab.deri.ie/search",
  content: include_str!("../prefixes/search.ttl"),
  name: "search",
  title: "Sindice Search Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/seas/",
  content: include_str!("../prefixes/seas.ttl"),
  name: "seas",
  title: "SEAS ontology",
}, LocalPrefix {
  location: "https://w3id.org/seas/EvaluationOntology",
  content: include_str!("../prefixes/seas-eval.ttl"),
  name: "seas-eval",
  title: "The SEAS Evaluation ontology",
}, LocalPrefix {
  location: "https://w3id.org/seas/OperatingOntology",
  content: include_str!("../prefixes/seas-op.ttl"),
  name: "seas-op",
  title: "The SEAS Failable System ontology",
}, LocalPrefix {
  location: "https://w3id.org/seas/QUDTAlignment",
  content: include_str!("../prefixes/seas-qudt.ttl"),
  name: "seas-qudt",
  title: "QUDT Alignment.",
}, LocalPrefix {
  location: "https://w3id.org/seas/StatisticsOntology",
  content: include_str!("../prefixes/seas-stats.ttl"),
  name: "seas-stats",
  title: "The SEAS Statistics ontology.",
}, LocalPrefix {
  location: "https://w3id.org/seas/SystemOntology",
  content: include_str!("../prefixes/seas-sys.ttl"),
  name: "seas-sys",
  title: "The SEAS System ontology",
}, LocalPrefix {
  location: "https://w3id.org/seas/BatteryOntology",
  content: include_str!("../prefixes/seasb.ttl"),
  name: "seasb",
  title: "The SEAS Battery ontology.",
}, LocalPrefix {
  location: "https://w3id.org/seas/BuildingOntology",
  content: include_str!("../prefixes/seasbo.ttl"),
  name: "seasbo",
  title: "The SEAS Building Ontology",
}, LocalPrefix {
  location: "https://w3id.org/seas/DeviceOntology",
  content: include_str!("../prefixes/seasd.ttl"),
  name: "seasd",
  title: "The SEAS Device ontology",
}, LocalPrefix {
  location: "https://w3id.org/seas/ForecastingOntology",
  content: include_str!("../prefixes/seasfo.ttl"),
  name: "seasfo",
  title: "The SEAS Forecasting ontology",
}, LocalPrefix {
  location: "https://w3id.org/seas/TimeOntology",
  content: include_str!("../prefixes/seast.ttl"),
  name: "seast",
  title: "The SEAS Time Ontology.",
}, LocalPrefix {
  location: "https://w3id.org/seas/TradingOntology",
  content: include_str!("../prefixes/seasto.ttl"),
  name: "seasto",
  title: "The SEAS Trading ontology",
}, LocalPrefix {
  location: "http://securitytoolbox.appspot.com/securityMain",
  content: include_str!("../prefixes/security.ttl"),
  name: "security",
  title: "Security Ontology",
}, LocalPrefix {
  location: "http://semanticweb.cs.vu.nl/2009/11/sem/",
  content: include_str!("../prefixes/sem.ttl"),
  name: "sem",
  title: "The SEM Ontology",
}, LocalPrefix {
  location: "http://www.lingvoj.org/semio.rdf",
  content: include_str!("../prefixes/semio.ttl"),
  name: "semio",
  title: "Semio, an ontology of signs",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/cp/owl/semiotics.owl",
  content: include_str!("../prefixes/semiotics.ttl"),
  name: "semiotics",
  title: "A content ontology pattern that encodes a basic semiotic theory, by reusing the situation pattern.",
}, LocalPrefix {
  location: "http://purl.org/SemSur/",
  content: include_str!("../prefixes/semsur.ttl"),
  name: "semsur",
  title: "The Semantic Survey Ontology (semsur)",
}, LocalPrefix {
  location: "https://w3id.org/seo",
  content: include_str!("../prefixes/seo.ttl"),
  name: "seo",
  title: "The Scientific Events Ontology",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/cp/owl/sequence.owl",
  content: include_str!("../prefixes/seq.ttl"),
  name: "seq",
  title: "Sequence Pattern",
}, LocalPrefix {
  location: "http://purl.org/ontology/service",
  content: include_str!("../prefixes/service.ttl"),
  name: "service",
  title: "The Service Ontology",
}, LocalPrefix {
  location: "http://www.opengis.net/ont/sf",
  content: include_str!("../prefixes/sf.ttl"),
  name: "sf",
  title: "Simplified Features Geometry",
}, LocalPrefix {
  location: "http://www.w3.org/ns/shacl#",
  content: include_str!("../prefixes/sh.ttl"),
  name: "sh",
  title: "W3C Shapes Constraint Language (SHACL) Vocabulary",
}, LocalPrefix {
  location: "http://dati.cdec.it/lod/shoah/",
  content: include_str!("../prefixes/shoah.ttl"),
  name: "shoah",
  title: "Shoah Vocabulary Specification",
}, LocalPrefix {
  location: "http://paul.staroch.name/thesis/SmartHomeWeather.owl#",
  content: include_str!("../prefixes/shw.ttl"),
  name: "shw",
  title: "Smart Home Weather",
}, LocalPrefix {
  location: "http://purl.org/ontology/similarity/",
  content: include_str!("../prefixes/sim.ttl"),
  name: "sim",
  title: "The Similarity Ontology",
}, LocalPrefix {
  location: "https://www.w3id.org/simulation/ontology/",
  content: include_str!("../prefixes/simu.ttl"),
  name: "simu",
  title: "Simulation Ontology",
}, LocalPrefix {
  location: "http://semanticscience.org/ontology/sio.owl",
  content: include_str!("../prefixes/sio.ttl"),
  name: "sio",
  title: "Semanticscience Integrated Ontology",
}, LocalPrefix {
  location: "http://rdfs.org/sioc/ns#",
  content: include_str!("../prefixes/sioc.ttl"),
  name: "sioc",
  title: "Semantically-Interlinked Online Communities",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/cp/owl/situation.owl",
  content: include_str!("../prefixes/situ.ttl"),
  name: "situ",
  title: "Situation Pattern",
}, LocalPrefix {
  location: "http://www.w3.org/2004/02/skos/core",
  content: include_str!("../prefixes/skos.ttl"),
  name: "skos",
  title: "Simple Knowledge Organization System",
}, LocalPrefix {
  location: "http://www.w3.org/2008/05/skos-xl",
  content: include_str!("../prefixes/skosxl.ttl"),
  name: "skosxl",
  title: "SKOS eXtension for Labels",
}, LocalPrefix {
  location: "http://ns.cerise-project.nl/energy/def/cim-smartgrid",
  content: include_str!("../prefixes/smg.ttl"),
  name: "smg",
  title: "CERISE CIM Profile for Smart Grids",
}, LocalPrefix {
  location: "http://rdf.myexperiment.org/ontologies/snarm/",
  content: include_str!("../prefixes/snarm.ttl"),
  name: "snarm",
  title: "Simple Network Access Rights Management Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/ns/solid/terms",
  content: include_str!("../prefixes/solid.ttl"),
  name: "solid",
  title: "Solid terms",
}, LocalPrefix {
  location: "http://purl.org/net/soron",
  content: include_str!("../prefixes/sor.ttl"),
  name: "sor",
  title: "SORON: Social Relationships ONtology",
}, LocalPrefix {
  location: "http://www.w3.org/ns/sosa/",
  content: include_str!("../prefixes/sosa.ttl"),
  name: "sosa",
  title: "Sensor, Observation, Sample, and Actuator (SOSA) Ontology",
}, LocalPrefix {
  location: "http://qudt.org/2.1/vocab/sou",
  content: include_str!("../prefixes/sou.ttl"),
  name: "sou",
  title: "QUDT VOCAB Systems of Units Release 2.1.34",
}, LocalPrefix {
  location: "http://spinrdf.org/sp",
  content: include_str!("../prefixes/sp.ttl"),
  name: "sp",
  title: "SPIN SPARQL Syntax",
}, LocalPrefix {
  location: "http://geovocab.org/spatial",
  content: include_str!("../prefixes/spatial.ttl"),
  name: "spatial",
  title: "NeoGeo Spatial Ontology",
}, LocalPrefix {
  location: "http://spi-fm.uca.es/spdef/models/deployment/spcm/1.0",
  content: include_str!("../prefixes/spcm.ttl"),
  name: "spcm",
  title: "Software Process Control Model",
}, LocalPrefix {
  location: "http://kmi.open.ac.uk/projects/smartproducts/ontologies/food.owl",
  content: include_str!("../prefixes/spfood.ttl"),
  name: "spfood",
  title: "SmartProducts Food Domain Model",
}, LocalPrefix {
  location: "http://spinrdf.org/spin",
  content: include_str!("../prefixes/spin.ttl"),
  name: "spin",
  title: "SPIN Inferencing Vocabulary",
}, LocalPrefix {
  location: "http://www.bbc.co.uk/ontologies/sport",
  content: include_str!("../prefixes/sport.ttl"),
  name: "sport",
  title: "BBC Sport Ontology",
}, LocalPrefix {
  location: "http://spitfire-project.eu/ontology/ns",
  content: include_str!("../prefixes/spt.ttl"),
  name: "spt",
  title: "SPITFIRE Ontology",
}, LocalPrefix {
  location: "https://bmake.th-brandenburg.de/spv",
  content: include_str!("../prefixes/spvqa.ttl"),
  name: "spvqa",
  title: "Scholarly Papers Vocabulary with Focus on Qualtitative Analysis",
}, LocalPrefix {
  location: "http://ns.inria.fr/ast/sql#",
  content: include_str!("../prefixes/sql.ttl"),
  name: "sql",
  title: "SQL Abstract Syntax Trees Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/sri",
  content: include_str!("../prefixes/sri.ttl"),
  name: "sri",
  title: "Smart Readiness Indicator Vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/ns/ssn/",
  content: include_str!("../prefixes/ssn.ttl"),
  name: "ssn",
  title: "Semantic Sensor Network Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/ns/ssn/",
  content: include_str!("../prefixes/ssno.ttl"),
  name: "ssno",
  title: "Semantic Sensor Network Ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/ssso",
  content: include_str!("../prefixes/ssso.ttl"),
  name: "ssso",
  title: "Simple Service Status Ontology",
}, LocalPrefix {
  location: "http://semweb.mmlab.be/ns/stoptimes#Ontology",
  content: include_str!("../prefixes/st.ttl"),
  name: "st",
  title: "The Stop Times ontology",
}, LocalPrefix {
  location: "http://securitytoolbox.appspot.com/stac",
  content: include_str!("../prefixes/stac.ttl"),
  name: "stac",
  title: "Security Toolbox : Attacks and Countermeasures",
}, LocalPrefix {
  location: "https://w3id.org/stax/ontology",
  content: include_str!("../prefixes/stax.ttl"),
  name: "stax",
  title: "RDF Stream Taxonomy (RDF-STaX)",
}, LocalPrefix {
  location: "http://purl.org/net/step",
  content: include_str!("../prefixes/step.ttl"),
  name: "step",
  title: "Semantic Trajectory Episodes",
}, LocalPrefix {
  location: "https://w3id.org/i40/sto#",
  content: include_str!("../prefixes/sto.ttl"),
  name: "sto",
  title: "i40 Standards Lanscape Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/ontology/stories/",
  content: include_str!("../prefixes/stories.ttl"),
  name: "stories",
  title: "Stories Ontology",
}, LocalPrefix {
  location: "http://purl.org/voc/summa/",
  content: include_str!("../prefixes/summa.ttl"),
  name: "summa",
  title: "SUMMA Vocabulary",
}, LocalPrefix {
  location: "https://www.w3id.org/survey-ontology",
  content: include_str!("../prefixes/sur.ttl"),
  name: "sur",
  title: "The Survey Ontology",
}, LocalPrefix {
  location: "https://w3id.org/squap/",
  content: include_str!("../prefixes/sw-quality.ttl"),
  name: "sw-quality",
  title: "SQuAP Ontology",
}, LocalPrefix {
  location: "http://data.semanticweb.org/ns/swc/ontology",
  content: include_str!("../prefixes/swc.ttl"),
  name: "swc",
  title: "Semantic Web Conference Ontology",
}, LocalPrefix {
  location: "https://w3id.org/semsys/ns/swemls",
  content: include_str!("../prefixes/swemls.ttl"),
  name: "swemls",
  title: "Semantic-Web Machine Learning System (SWeMLS) Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2004/03/trix/swp-1",
  content: include_str!("../prefixes/swp.ttl"),
  name: "swp",
  title: "Graph Authority",
}, LocalPrefix {
  location: "http://spi-fm.uca.es/spdef/models/deployment/swpm/1.0",
  content: include_str!("../prefixes/swpm.ttl"),
  name: "swpm",
  title: "Software Work Product Model",
}, LocalPrefix {
  location: "http://sw-portal.deri.org/ontologies/swportal",
  content: include_str!("../prefixes/swpo.ttl"),
  name: "swpo",
  title: "Semantic Web Portal Ontology",
}, LocalPrefix {
  location: "http://swrc.ontoware.org/ontology-07",
  content: include_str!("../prefixes/swrc.ttl"),
  name: "swrc",
  title: "Semantic Web for Research Communities",
}, LocalPrefix {
  location: "http://www.w3.org/2003/11/swrl",
  content: include_str!("../prefixes/swrl.ttl"),
  name: "swrl",
  title: "Semantic Web Rule Language",
}, LocalPrefix {
  location: "http://www.w3.org/ns/lemon/synsem",
  content: include_str!("../prefixes/synsem.ttl"),
  name: "synsem",
  title: "Lexicon Model for Ontologies - Synsem",
}, LocalPrefix {
  location: "http://ns.bergnet.org/tac/0.1/triple-access-control",
  content: include_str!("../prefixes/tac.ttl"),
  name: "tac",
  title: "TripleAccessControl Ontology",
}, LocalPrefix {
  location: "http://www.holygoat.co.uk/owl/redwood/0.1/tags/",
  content: include_str!("../prefixes/tag.ttl"),
  name: "tag",
  title: "Tag ontology",
}, LocalPrefix {
  location: "http://vocab.deri.ie/tao",
  content: include_str!("../prefixes/tao.ttl"),
  name: "tao",
  title: "Trust Assertion Ontology",
}, LocalPrefix {
  location: "https://w3id.org/EUTaxO",
  content: include_str!("../prefixes/tax.ttl"),
  name: "tax",
  title: "EUTaxO - EUdaphobase Taxonomy Ontology",
}, LocalPrefix {
  location: "http://purl.org/biodiversity/taxon/",
  content: include_str!("../prefixes/taxon.ttl"),
  name: "taxon",
  title: "TaxonMap Ontology",
}, LocalPrefix {
  location: "http://taxref.mnhn.fr/lod/taxref-ld",
  content: include_str!("../prefixes/taxref-ld.ttl"),
  name: "taxref-ld",
  title: "TAXREF-LD Ontology",
}, LocalPrefix {
  location: "https://w3id.org/timebank",
  content: include_str!("../prefixes/tb.ttl"),
  name: "tb",
  title: "Timebank Ontology",
}, LocalPrefix {
  location: "https://www.w3.org/2019/wot/td",
  content: include_str!("../prefixes/td.ttl"),
  name: "td",
  title: "Thing Description Ontology",
}, LocalPrefix {
  location: "https://w3id.org/todo/tododfa",
  content: include_str!("../prefixes/tddfa.ttl"),
  name: "tddfa",
  title: "TODODFA: Frame-Action Module for Task-Oriented Dialogue management Ontology (TODO)",
}, LocalPrefix {
  location: "https://w3id.org/todo/tododial",
  content: include_str!("../prefixes/tddial.ttl"),
  name: "tddial",
  title: "TODODial: Dialogue Module for Task-Oriented Dialogue management Ontology (TODO)",
}, LocalPrefix {
  location: "https://w3id.org/todo/tododm",
  content: include_str!("../prefixes/tddm.ttl"),
  name: "tddm",
  title: "TODODM: Dialogue Management Module for Task-Oriented Dialogue management Ontology (TODO)",
}, LocalPrefix {
  location: "https://w3id.org/todo/tododom",
  content: include_str!("../prefixes/tddom.ttl"),
  name: "tddom",
  title: "TODODom: Domain Module for Task-Oriented Dialogue management Ontology (TODO)",
}, LocalPrefix {
  location: "https://w3id.org/todo/tododt",
  content: include_str!("../prefixes/tddt.ttl"),
  name: "tddt",
  title: "TODODT: Dialogue Tracing Module for Task-Oriented Dialogue management Ontology (TODO)",
}, LocalPrefix {
  location: "https://w3id.org/todo/tododw",
  content: include_str!("../prefixes/tddw.ttl"),
  name: "tddw",
  title: "TODODW: World Module for Task-Oriented Dialogue management Ontology (TODO)",
}, LocalPrefix {
  location: "http://www.w3.org/2006/time-entry",
  content: include_str!("../prefixes/te.ttl"),
  name: "te",
  title: "Time Entry",
}, LocalPrefix {
  location: "http://linkedscience.org/teach/ns#",
  content: include_str!("../prefixes/teach.ttl"),
  name: "teach",
  title: "Teaching Core Vocabulary Specification",
}, LocalPrefix {
  location: "http://purl.org/tempo",
  content: include_str!("../prefixes/tempo.ttl"),
  name: "tempo",
  title: "TempO - Temporal Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2006/03/test-description",
  content: include_str!("../prefixes/test.ttl"),
  name: "test",
  title: "Test Metadata",
}, LocalPrefix {
  location: "http://purl.org/theatre#",
  content: include_str!("../prefixes/theatre.ttl"),
  name: "theatre",
  title: "Theatre Ontology",
}, LocalPrefix {
  location: "http://resource.geosciml.org/ontology/timescale/thors",
  content: include_str!("../prefixes/thors.ttl"),
  name: "thors",
  title: "The Temporal Ordinal Reference Systems",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/cp/owl/timeinterval.owl",
  content: include_str!("../prefixes/ti.ttl"),
  name: "ti",
  title: "The Time Interval Pattern",
}, LocalPrefix {
  location: "http://www.w3.org/2006/time",
  content: include_str!("../prefixes/time.ttl"),
  name: "time",
  title: "Time Ontology",
}, LocalPrefix {
  location: "http://purl.org/tio/ns#",
  content: include_str!("../prefixes/tio.ttl"),
  name: "tio",
  title: "The Tickets Ontology",
}, LocalPrefix {
  location: "http://www.ontologydesignpatterns.org/cp/owl/timeindexedsituation.owl",
  content: include_str!("../prefixes/tis.ttl"),
  name: "tis",
  title: "Time Indexed Situation",
}, LocalPrefix {
  location: "http://www.observedchange.com/tisc/ns#",
  content: include_str!("../prefixes/tisc.ttl"),
  name: "tisc",
  title: "Open Time and Space Core Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/NET/c4dm/timeline.owl",
  content: include_str!("../prefixes/tl.ttl"),
  name: "tl",
  title: "The Timeline Ontology",
}, LocalPrefix {
  location: "http://def.seegrid.csiro.au/isotc211/iso19108/2002/temporal",
  content: include_str!("../prefixes/tm.ttl"),
  name: "tm",
  title: "A vocabulary for temporal objects in Geographic Information",
}, LocalPrefix {
  location: "http://www.w3.org/2001/sw/hcls/ns/transmed/",
  content: include_str!("../prefixes/tmo.ttl"),
  name: "tmo",
  title: "Translational Medicine Ontology",
}, LocalPrefix {
  location: "http://purl.org/toco/",
  content: include_str!("../prefixes/toco.ttl"),
  name: "toco",
  title: "Toucan Ontology",
}, LocalPrefix {
  location: "https://w3id.org/todo",
  content: include_str!("../prefixes/todo.ttl"),
  name: "todo",
  title: "TODO: Task-Oriented Dialogue management Ontology",
}, LocalPrefix {
  location: "http://data.ign.fr/def/topo",
  content: include_str!("../prefixes/topo.ttl"),
  name: "topo",
  title: "An ontology for describing territory elements an infrastructure at French Mapping Agency.",
}, LocalPrefix {
  location: "http://tour-pedia.org/download/tp.owl",
  content: include_str!("../prefixes/tp.ttl"),
  name: "tp",
  title: "Tourpedia Ontology",
}, LocalPrefix {
  location: "http://www.sensormeasurement.appspot.com/ont/transport/traffic",
  content: include_str!("../prefixes/traffic.ttl"),
  name: "traffic",
  title: "Road Traffic Management",
}, LocalPrefix {
  location: "http://contextus.net/ontology/ontomedia/ext/common/trait#",
  content: include_str!("../prefixes/trait.ttl"),
  name: "trait",
  title: "OntoMedia Trait Representation",
}, LocalPrefix {
  location: "http://vocab.org/transit/terms/",
  content: include_str!("../prefixes/transit.ttl"),
  name: "transit",
  title: "TRANSIT",
}, LocalPrefix {
  location: "http://linkeddata.finki.ukim.mk/lod/ontology/tao#",
  content: include_str!("../prefixes/trao.ttl"),
  name: "trao",
  title: "Transport Administration Ontology",
}, LocalPrefix {
  location: "https://w3id.org/tree",
  content: include_str!("../prefixes/tree.ttl"),
  name: "tree",
  title: "TREE",
}, LocalPrefix {
  location: "https://liidr.org/trust-recommendation-in-social-internet-of-things/",
  content: include_str!("../prefixes/tresiot.ttl"),
  name: "tresiot",
  title: "Ontology for Trust Recommendation in Social Internet of Things",
}, LocalPrefix {
  location: "https://w3id.org/tribont",
  content: include_str!("../prefixes/tribont.ttl"),
  name: "tribont",
  title: "TribOnt ontology",
}, LocalPrefix {
  location: "https://w3id.org/tribont/core",
  content: include_str!("../prefixes/tribont-core.ttl"),
  name: "tribont-core",
  title: "Core module",
}, LocalPrefix {
  location: "https://w3id.org/tribont/equipment",
  content: include_str!("../prefixes/tribont-equipment.ttl"),
  name: "tribont-equipment",
  title: "Equipment module",
}, LocalPrefix {
  location: "https://w3id.org/tribont/material",
  content: include_str!("../prefixes/tribont-material.ttl"),
  name: "tribont-material",
  title: "Material module",
}, LocalPrefix {
  location: "https://w3id.org/tribont/sample",
  content: include_str!("../prefixes/tribont-sample.ttl"),
  name: "tribont-sample",
  title: "Sample module",
}, LocalPrefix {
  location: "https://w3id.org/TRO",
  content: include_str!("../prefixes/tro.ttl"),
  name: "tro",
  title: "Transparent Relations Ontology",
}, LocalPrefix {
  location: "http://rdfs.org/sioc/types#",
  content: include_str!("../prefixes/tsioc.ttl"),
  name: "tsioc",
  title: "SIOC Types Ontology Module",
}, LocalPrefix {
  location: "http://purl.org/net/tsn#",
  content: include_str!("../prefixes/tsn.ttl"),
  name: "tsn",
  title: "Territorial Statistical Nomenclature Ontology",
}, LocalPrefix {
  location: "http://purl.org/net/tsnchange#",
  content: include_str!("../prefixes/tsnc.ttl"),
  name: "tsnc",
  title: "Territorial Statistical Nomenclature Change Ontology",
}, LocalPrefix {
  location: "http://idi.fundacionctic.org/cruzar/turismo",
  content: include_str!("../prefixes/turismo.ttl"),
  name: "turismo",
  title: "Ontology of Tourist for Saragossa town hall.",
}, LocalPrefix {
  location: "http://www.essepuntato.it/2012/04/tvc",
  content: include_str!("../prefixes/tvc.ttl"),
  name: "tvc",
  title: "The Time-indexed Value in Context",
}, LocalPrefix {
  location: "http://lod.taxonconcept.org/ontology/txn.owl",
  content: include_str!("../prefixes/txn.ttl"),
  name: "txn",
  title: "TaxonConcept Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2006/timezone",
  content: include_str!("../prefixes/tzont.ttl"),
  name: "tzont",
  title: "Time Zone Ontology",
}, LocalPrefix {
  location: "http://purl.org/olia/ubyCat.owl",
  content: include_str!("../prefixes/uby.ttl"),
  name: "uby",
  title: "ubyCat.owl",
}, LocalPrefix {
  location: "http://purl.org/uco/ns#",
  content: include_str!("../prefixes/uco.ttl"),
  name: "uco",
  title: "Used Cars Ontology",
}, LocalPrefix {
  location: "http://purl.oclc.org/NET/muo/ucum/",
  content: include_str!("../prefixes/ucum.ttl"),
  name: "ucum",
  title: "Units of measurement ontology",
}, LocalPrefix {
  location: "http://www.w3.org/ns/ui",
  content: include_str!("../prefixes/ui.ttl"),
  name: "ui",
  title: "A user interface ontology",
}, LocalPrefix {
  location: "http://www.w3id.org/urban-iot/core",
  content: include_str!("../prefixes/uiot.ttl"),
  name: "uiot",
  title: "Urban IoT Ontologies - Core Module",
}, LocalPrefix {
  location: "http://www.w3id.org/urban-iot/electric",
  content: include_str!("../prefixes/uiote.ttl"),
  name: "uiote",
  title: "Urban IoT Ontologies - Electric Mobility Module",
}, LocalPrefix {
  location: "http://umbel.org/umbel",
  content: include_str!("../prefixes/umbel.ttl"),
  name: "umbel",
  title: "Upper Mapping and Binding Exchange Layer",
}, LocalPrefix {
  location: "http://purl.org/umu/uneskos",
  content: include_str!("../prefixes/uneskos.ttl"),
  name: "uneskos",
  title: "UNESKOS Vocabulary",
}, LocalPrefix {
  location: "http://purl.uniprot.org/core/",
  content: include_str!("../prefixes/uniprot.ttl"),
  name: "uniprot",
  title: "Uniprot Core Ontology",
}, LocalPrefix {
  location: "http://uri4uri.net/vocab",
  content: include_str!("../prefixes/uri4uri.ttl"),
  name: "uri4uri",
  title: "URI Vocabulary",
}, LocalPrefix {
  location: "https://w3id.org/usability",
  content: include_str!("../prefixes/usability.ttl"),
  name: "usability",
  title: "Usability",
}, LocalPrefix {
  location: "http://code-research.eu/ontology/visual-analytics",
  content: include_str!("../prefixes/va.ttl"),
  name: "va",
  title: "The Visual Analytics Vocabulary",
}, LocalPrefix {
  location: "http://www.linkedmodel.org/schema/vaem",
  content: include_str!("../prefixes/vaem.ttl"),
  name: "vaem",
  title: "Vocabulary for Attaching Essential Metadata",
}, LocalPrefix {
  location: "http://www.essepuntato.it/2013/10/vagueness",
  content: include_str!("../prefixes/vag.ttl"),
  name: "vag",
  title: "The Vagueness Ontology",
}, LocalPrefix {
  location: "https://w3id.org/vair",
  content: include_str!("../prefixes/vair.ttl"),
  name: "vair",
  title: "Vocabulary of AI Risks",
}, LocalPrefix {
  location: "http://purl.org/vocab/vann/",
  content: include_str!("../prefixes/vann.ttl"),
  name: "vann",
  title: "VANN: A vocabulary for annotating vocabulary descriptions",
}, LocalPrefix {
  location: "http://www.w3.org/ns/lemon/vartrans",
  content: include_str!("../prefixes/vartrans.ttl"),
  name: "vartrans",
  title: "Lexicon Model for Ontologies - Vartrans",
}, LocalPrefix {
  location: "http://www.w3.org/2006/vcard/ns",
  content: include_str!("../prefixes/vcard.ttl"),
  name: "vcard",
  title: "An Ontology for vCards",
}, LocalPrefix {
  location: "http://data.lirmm.fr/ontologies/vdpp",
  content: include_str!("../prefixes/vdpp.ttl"),
  name: "vdpp",
  title: "Vocabulary for Dataset Publication Projects",
}, LocalPrefix {
  location: "http://linkeddata.finki.ukim.mk/lod/ontology/veo#",
  content: include_str!("../prefixes/veo.ttl"),
  name: "veo",
  title: "Vehicle Emissions Ontology",
}, LocalPrefix {
  location: "http://purl.org/net/VideoGameOntology",
  content: include_str!("../prefixes/vgo.ttl"),
  name: "vgo",
  title: "The Video Game Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/TR/2003/PR-owl-guide-20031209/wine",
  content: include_str!("../prefixes/vin.ttl"),
  name: "vin",
  title: "Wine Ontology",
}, LocalPrefix {
  location: "http://w3id.org/vir#",
  content: include_str!("../prefixes/vir.ttl"),
  name: "vir",
  title: "VIR - Visual Representation ontology",
}, LocalPrefix {
  location: "http://vivoweb.org/ontology/core",
  content: include_str!("../prefixes/vivo.ttl"),
  name: "vivo",
  title: "VIVO Core Ontology",
}, LocalPrefix {
  location: "http://spi-fm.uca.es/spdef/models/genericTools/vmm/1.0",
  content: include_str!("../prefixes/vmm.ttl"),
  name: "vmm",
  title: "Visual Modeling tool Model",
}, LocalPrefix {
  location: "http://purl.org/vocommons/voaf",
  content: include_str!("../prefixes/voaf.ttl"),
  name: "voaf",
  title: "Vocabulary of a Friend",
}, LocalPrefix {
  location: "http://voag.linkedmodel.org/schema/voag",
  content: include_str!("../prefixes/voag.ttl"),
  name: "voag",
  title: "Vocabulary Of Attribution and Governance",
}, LocalPrefix {
  location: "http://w3id.org/rsp/vocals#",
  content: include_str!("../prefixes/vocals.ttl"),
  name: "vocals",
  title: "VoCaLS: A Vocabulary and Catalog for Linked Streams",
}, LocalPrefix {
  location: "http://vocab.deri.ie/void",
  content: include_str!("../prefixes/void.ttl"),
  name: "void",
  title: "Vocabulary of Interlinked Datasets",
}, LocalPrefix {
  location: "http://purl.org/query/voidext",
  content: include_str!("../prefixes/voidext.ttl"),
  name: "voidext",
  title: "Extended Vocabulary of Interlinked Datasets (VoIDext)",
}, LocalPrefix {
  location: "http://www.ics.forth.gr/isl/VoIDWarehouse/VoID_Extension_Schema.owl",
  content: include_str!("../prefixes/voidwh.ttl"),
  name: "voidwh",
  title: "Void Warehouse Ontology",
}, LocalPrefix {
  location: "http://simile.mit.edu/2003/10/ontologies/vraCore3#",
  content: include_str!("../prefixes/vra.ttl"),
  name: "vra",
  title: "RDFS Ontology for VRA",
}, LocalPrefix {
  location: "http://vocab.sti2.at/vrank",
  content: include_str!("../prefixes/vrank.ttl"),
  name: "vrank",
  title: "Vocabulary for Ranking",
}, LocalPrefix {
  location: "http://www.w3.org/2003/06/sw-vocab-status/ns",
  content: include_str!("../prefixes/vs.ttl"),
  name: "vs",
  title: "SemWeb Vocab Status ontology",
}, LocalPrefix {
  location: "http://purl.org/vsearch/",
  content: include_str!("../prefixes/vsearch.ttl"),
  name: "vsearch",
  title: "vSearch Vocabulary",
}, LocalPrefix {
  location: "http://purl.org/vso/ns",
  content: include_str!("../prefixes/vso.ttl"),
  name: "vso",
  title: "Vehicle Sales Ontology",
}, LocalPrefix {
  location: "http://purl.org/vvo/ns#",
  content: include_str!("../prefixes/vvo.ttl"),
  name: "vvo",
  title: "Volkswagen Vehicles Ontology",
}, LocalPrefix {
  location: "https://www.w3.org/ns/ssn",
  content: include_str!("../prefixes/w3c-ssn.ttl"),
  name: "w3c-ssn",
  title: "Semantic Sensor Network Ontology",
}, LocalPrefix {
  location: "http://purl.org/wai#",
  content: include_str!("../prefixes/wai.ttl"),
  name: "wai",
  title: "Roles and Profiles Ontology",
}, LocalPrefix {
  location: "http://www.w3.org/2007/05/powder-s",
  content: include_str!("../prefixes/wdrs.ttl"),
  name: "wdrs",
  title: "Protocol for Web Description Resources",
}, LocalPrefix {
  location: "http://purl.org/net/wf-invocation",
  content: include_str!("../prefixes/wf-invoc.ttl"),
  name: "wf-invoc",
  title: "Workflow Invocation Ontology",
}, LocalPrefix {
  location: "http://purl.org/wf4ever/wfdesc",
  content: include_str!("../prefixes/wfdesc.ttl"),
  name: "wfdesc",
  title: "The Wfdesc ontology",
}, LocalPrefix {
  location: "http://purl.org/net/wf-motifs",
  content: include_str!("../prefixes/wfm.ttl"),
  name: "wfm",
  title: "The Workflow Motif Ontology",
}, LocalPrefix {
  location: "https://w3id.org/wfont",
  content: include_str!("../prefixes/wfont.ttl"),
  name: "wfont",
  title: "Wind Farm Ontology (wfont)",
}, LocalPrefix {
  location: "http://purl.org/wf4ever/wfprov",
  content: include_str!("../prefixes/wfprov.ttl"),
  name: "wfprov",
  title: "The Wfprov Ontology",
}, LocalPrefix {
  location: "http://vocab.org/whisky/terms",
  content: include_str!("../prefixes/whisky.ttl"),
  name: "whisky",
  title: "Whisky Ontology",
}, LocalPrefix {
  location: "http://www.kanzaki.com/ns/whois",
  content: include_str!("../prefixes/whois.ttl"),
  name: "whois",
  title: "Who's who description vocabulary",
}, LocalPrefix {
  location: "http://purl.org/ontology/wi/core#",
  content: include_str!("../prefixes/wi.ttl"),
  name: "wi",
  title: "The Weighted Interests Vocabulary",
}, LocalPrefix {
  location: "http://wikiba.se/ontology",
  content: include_str!("../prefixes/wikibase.ttl"),
  name: "wikibase",
  title: "Wikibase system ontology",
}, LocalPrefix {
  location: "http://spi-fm.uca.es/spdef/models/genericTools/wikim/1.0",
  content: include_str!("../prefixes/wikim.ttl"),
  name: "wikim",
  title: "WIKI tool Model",
}, LocalPrefix {
  location: "http://www.wsmo.org/ns/wsmo-lite#",
  content: include_str!("../prefixes/wl.ttl"),
  name: "wl",
  title: "WSMO-Lite Ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/wo/",
  content: include_str!("../prefixes/wlo.ttl"),
  name: "wlo",
  title: "BBC Wildlife Ontology",
}, LocalPrefix {
  location: "http://purl.org/ontology/wo/core#",
  content: include_str!("../prefixes/wo.ttl"),
  name: "wo",
  title: "Weighting Ontology",
}, LocalPrefix {
  location: "http://xmlns.com/wot/0.1/",
  content: include_str!("../prefixes/wot.ttl"),
  name: "wot",
  title: "Web Of Trust",
}, LocalPrefix {
  location: "https://www.w3.org/2019/wot/security#",
  content: include_str!("../prefixes/wotsec.ttl"),
  name: "wotsec",
  title: "Security mechanisms for the Web of Things",
}, LocalPrefix {
  location: "https://www.w3.org/ns/pim/space",
  content: include_str!("../prefixes/ws.ttl"),
  name: "ws",
  title: "An ontology for describing Workspaces.",
}, LocalPrefix {
  location: "http://purl.org/xapi/ontology#",
  content: include_str!("../prefixes/xapi.ttl"),
  name: "xapi",
  title: "xAPI Controlled Vocabulary Ontology",
}, LocalPrefix {
  location: "https://w3id.org/vocab/xbrll",
  content: include_str!("../prefixes/xbrll.ttl"),
  name: "xbrll",
  title: "A lightweight XBRL vocabulary",
}, LocalPrefix {
  location: "http://www.w3.org/1999/xhtml/vocab",
  content: include_str!("../prefixes/xhv.ttl"),
  name: "xhv",
  title: "XHTML Vocabulary",
}, LocalPrefix {
  location: "http://rdf-vocabulary.ddialliance.org/xkos",
  content: include_str!("../prefixes/xkos.ttl"),
  name: "xkos",
  title: "XKOS",
}, LocalPrefix {
  location: "https://yogaontology.org/ontology/",
  content: include_str!("../prefixes/yoga.ttl"),
  name: "yoga",
  title: "Yoga Ontology",
}, LocalPrefix {
  location: "http://zbw.eu/namespaces/zbw-extensions",
  content: include_str!("../prefixes/zbwext.ttl"),
  name: "zbwext",
  title: "ZBW Extensions",
}];
