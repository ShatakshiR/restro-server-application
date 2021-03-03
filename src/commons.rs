use mongodb::bson;
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Counters{
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: String,
    pub sequence_value: isize
}

pub fn struct_to_document<'a, T: Sized + Serialize + Deserialize<'a>>(t: &T) -> Option<Document> {
    let mid: Option<Document> = bson::to_bson(t)
        .ok()
        .map(|x| x.as_document().unwrap().to_owned());

    mid.map(|mut doc| {
        let keys = doc.keys();
        let rm: Vec<String> = keys
            .filter(|k| doc.is_null(k))
            .map(|x| x.to_owned())
            .collect();
        // remove null value fields
        for x in rm {
            doc.remove(&x);
        }
        doc
    })
}

