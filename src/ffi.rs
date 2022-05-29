use std::{error, any::Any};

use dyn_clonable::*;

use dlopen::symbor::Library;

use crate::{
    mem::{StackMemory, HashMemory, Object},
    runtime::Trace, error::Error,
};

pub fn load_library(name: &str, stack: &mut StackMemory, hashs: &mut HashMemory) -> Result<Library, Box<dyn error::Error>> {
    let lib = Library::open(name)?;
    let trl_init = unsafe { lib.symbol::<fn(&mut StackMemory, &mut HashMemory)>("trl_init")? };
    trl_init(stack, hashs);
    Ok(lib)
}

pub fn terminate_library(name: &str, lib: Library, stack: &mut StackMemory, hashs: &mut HashMemory) -> Result<(), Box<dyn error::Error>> {
    let trl_term_s = unsafe { lib.symbol::<fn(&mut StackMemory, &mut HashMemory)>("trl_term") };
    match trl_term_s {
        Ok(trl_term) => {
            trl_term(stack, hashs);
        }
        Err(dlopen::Error::SymbolGettingError(e)) if format!("{}", e) == format!("{}: undefined symbol: trl_term", name) => (),
        Err(e) => {
            return Err(Box::new(e));
        }
    }
    Ok(())
}

#[clonable]
pub trait FfiObject: Clone + 'static {
    fn repr(&self) -> String;
    fn as_any(&'static self) -> &'static dyn Any where Self: Sized {
        self
    }
    fn extract<T>(&'static self) -> Option<&T>
    where
        T: FfiObject + 'static,
        Self: Sized
    {
        self.as_any().downcast_ref::<T>()
    }
    fn destroy(&mut self);

    fn add(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }
    fn substract(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }
    fn multiply(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }
    fn divide(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }
    fn modulo(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }

    fn equal(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }
    fn not_equal(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }
    fn greater(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }
    fn lesser(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }
    fn greater_eq(&self, o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        let gt = self.greater(o.clone(), trace)?;
        let eq = self.equal(o, trace)?;
        Ok(if matches!(gt, Object::Bool(true))
            || matches!(eq, Object::Bool(true)) {
            Object::Bool(true)
        } else { Object::Bool(false) })
    }
    fn lesser_eq(&self, o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        let gt = self.lesser(o.clone(), trace)?;
        let eq = self.equal(o, trace)?;
        Ok(if matches!(gt, Object::Bool(true))
            || matches!(eq, Object::Bool(true)) {
            Object::Bool(true)
        } else { Object::Bool(false) })
    }
    fn not(&self, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }

    fn and(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }
    fn or(&self, _o: Object, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new("DesteklenmeyenOperasyon", "operasyon desteklenmiyor", trace.clone(), None))
    }

    fn access(&self, s: String, trace: &Vec<Trace>) -> Result<Object, Error> {
        Err(Error::new(
            "BilinmeyenTanımlayıcı",
            &format!(
                "bilinmeyen değişken: `{}`, bu değişken bulunamamıştır",
                s,
            ),
            trace.clone(),
            None,
        ))
    }
}

#[derive(Clone)]
pub struct FfiFunction(pub Box<fn(&mut StackMemory, &mut HashMemory) -> Result<Option<Object>, Error>>);

impl FfiFunction {
    pub fn call(&self, s: &mut StackMemory, h: &mut HashMemory) -> Result<Option<Object>, Error> {
        self.fun()(s, h)
    }
    pub fn fun(&self) -> &Box<fn(&mut StackMemory, &mut HashMemory) -> Result<Option<Object>, Error>> {
        &self.0
    }
}

#[macro_export]
macro_rules! make_function {
    ($f:expr) => {
        $crate::mem::Object::FfiFunction($crate::ffi::FfiFunction(Box::new($f)))
    };
}

#[macro_export]
macro_rules! make_object {
    ($o:expr) => {
        $crate::mem::Object::FfiObject(Box::new($o))
    };
}

