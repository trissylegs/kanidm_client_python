
use pyo3::{
    types::{
        PyDict,
    },
    exceptions::{
        PyException,
    },
    prelude::*
};
// use log::{
//     debug,
//     error,
//     info,
//     trace,
//     warn,
// };
// use uuid::Uuid;
// use time::OffsetDateTime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[no_mangle]
pub fn test_thing() {
    println!("Hello, world!");
}

#[pymodule]
pub fn kanidm_client(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    
    m.add_class::<KanidmClient>()?;
    
    Ok(())
}

#[pyclass]
pub struct KanidmClient {
    inner: kanidm_client::KanidmClient,
}

#[derive(Debug)]
enum KanidmError {
    ReqwestError(reqwest::Error),
    ClientError(kanidm_client::ClientError),
}
impl std::convert::From<KanidmError> for PyErr {
    fn from(error: KanidmError) -> PyErr {
        match error {
            KanidmError::ReqwestError(e) => PyException::new_err(e.to_string()),
            KanidmError::ClientError(e) => PyException::new_err(format!("{:?}", e)),                
        }
    }
}

#[pymethods]
impl KanidmClient {
    #[new]
    fn new() -> PyResult<Self> {
        let inner = kanidm_client::KanidmClientBuilder::new()
            .read_options_from_optional_config("/etc/kanidm/config")
            .expect("read_options_from_optional_config")
            .build()
            .map_err(KanidmError::ReqwestError)?;
        Ok(Self {
            inner
        })
    }

    #[getter]
    fn origin(&self) -> &str {
        self.inner.get_origin()
    }

    #[getter]
    fn url(&self) -> &str {
        self.inner.get_url()
    }

    #[setter]
    fn set_token(&self, token: String) {
        self.inner.set_token(token)
    }
    
    #[getter]
    fn get_token(&self) -> Option<String> {
        self.inner.get_token()
    }

    fn whoami(&self) -> PyResult<Option<(Entry, UserAuthToken)>> {
        match self.inner.whoami() {
            Err(error) => Err(KanidmError::ClientError(error))?,
            Ok(None) => Ok(None),
            Ok(Some((entry, token))) => {
                let entry = Entry(entry);
                let token = UserAuthToken(token);
                Ok(Some((entry, token)))
            }
        }
    }
}

struct Entry(kanidm_proto::v1::Entry);

impl ToPyObject for Entry {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.0.attrs.to_object(py)
    }
}
impl IntoPy<PyObject> for Entry {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}

struct UserAuthToken(kanidm_proto::v1::UserAuthToken);
/*
    pub session_id: Uuid,
    pub auth_type: AuthType,
    pub expiry: OffsetDateTime,
    pub uuid: Uuid,
    pub spn: String,
    pub lim_uidx: bool,
    pub lim_rmax: usize,
    pub lim_pmax: usize,
    pub lim_fmax: usize,
*/


// impl IntoPy<Py<PyDict>> for UserAuthToken
impl ToPyObject for UserAuthToken {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        // TODO: Figure out how to raise errors from to_object
        let dict = PyDict::new(py);
        let uuid = py.import("uuid").unwrap()
            .getattr("UUID").unwrap();
        let datetime = py.import("datetime")
            .unwrap();

        let datetime = datetime.getattr("datetime")
            .unwrap();
        let fromisoformat = datetime.getattr("fromisoformat")
            .unwrap();

        dict.set_item("session_id",
                      uuid.call1((self.0.session_id.to_string(),)).unwrap()).unwrap();

        dict.set_item("auth_type", format!("{:?}", self.0.auth_type)).unwrap();
        let expiry = self.0.expiry.format(time::Format::Rfc3339);
        dict.set_item("expiry", fromisoformat.call1((expiry,)).unwrap()).unwrap();
        dict.set_item("uuid", uuid.call1((self.0.uuid.to_string(),)).unwrap()).unwrap();
        dict.set_item("spn", &self.0.spn).unwrap();
        dict.set_item("lim_uidx", self.0.lim_uidx).unwrap();
        dict.set_item("lim_rmax", self.0.lim_rmax).unwrap();
        dict.set_item("lim_pmax", self.0.lim_pmax).unwrap();
        dict.set_item("lim_fmax", self.0.lim_fmax).unwrap();

        dict.to_object(py)
    }
}
impl IntoPy<PyObject> for UserAuthToken {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}
