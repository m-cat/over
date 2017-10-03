use std::sync::RwLock;

pub struct StringWrapper {
    inner: Arc<RwLock<String>>;
}

impl Clone for StringWrapper {
}

pub struct ArrWrapper {
    inner: Arc<RwLock<Vec<Value>>>;
}

impl Clone for ArrWrapper {

}

pub struct TupWrapper {
    inner: Arc<RwLock<Vec<Value>>>;
}

impl Clone for TupWrapper {

}
