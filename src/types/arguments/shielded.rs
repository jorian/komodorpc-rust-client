// for use in the `z_listoperationids` RPC
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Failed,
    Executing,
    Queued,
}

// for use in z_exportkey, z_exportviewingkey
#[derive(Debug, Serialize)]
pub struct ZAddr(String);

impl ZAddr {
    pub fn from(addr_str: &str) -> ZAddr {
        ZAddr { 0: addr_str.to_string() }
    }
}

// for use in z_exportwallet
#[derive(Debug, Serialize)]
pub struct ExportFileName(String);

impl ExportFileName {
    pub fn from(location: &str) -> ExportFileName {
        ExportFileName { 0: location.to_string() }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OperationIds(Vec<OperationId>);

impl OperationIds {
    pub fn from(vec: Vec<OperationId>) -> OperationIds {
        OperationIds{ 0: vec.to_owned() }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OperationId(String);

impl OperationId {
    // todo allow garbage in, garbage out? if not, how to check?
    pub fn from(id: &str) -> OperationId {
        OperationId { 0: id.to_string() }
    }
}