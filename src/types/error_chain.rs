use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use core::convert::Infallible;

pub trait ErrorPropogation<T, E> {
    fn on_error<C>(self, context: C) -> Result<T, ErrorChain>
    where C: Display + Send + Sync + 'static;

    fn do_on_error<C, F>(self, context_func: F) -> Result<T, ErrorChain>
    where C: Display + Send + Sync + 'static, F: FnOnce() -> C;
}

impl<T, E> ErrorPropogation<T, E> for Result<T, E>
where E: Error + Send + Sync + 'static {
    fn on_error<C>(self, context: C) -> Result<T, ErrorChain>
    where C: Display + Send + Sync + 'static {
        match self {
            Ok(val) => Ok(val),
            Err(error) => Err(ErrorChain::from(error, context))
        }
    }

    fn do_on_error<C, F>(self, context_func: F) -> Result<T, ErrorChain>
    where C: Display + Send + Sync + 'static, F: FnOnce() -> C {
        match self {
            Ok(val) => Ok(val),
            Err(error) => Err(ErrorChain::from(error, context_func()))
        }
    }
}

impl<T> ErrorPropogation<T, Infallible> for Option<T> {
    fn on_error<C>(self, context: C) -> Result<T, ErrorChain>
    where C: Display + Send + Sync + 'static {
        match self {
            Some(val) => Ok(val),
            None => Err(ErrorChain::new(context))
        }
    }

    fn do_on_error<C, F>(self, context_func: F) -> Result<T, ErrorChain>
    where C: Display + Send + Sync + 'static, F: FnOnce() -> C {
        match self {
            Some(val) => Ok(val),
            None => Err(ErrorChain::new(context_func()))
        }
    }
}

pub struct ErrorChain {
    context: Box<dyn Display + Sync + Send + 'static>,
    cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl ErrorChain {
    pub fn new<C>(context: C) -> ErrorChain 
    where C: Display + Sync + Send + 'static  {
        return ErrorChain { context: Box::new(context), cause: None }
    }

    pub fn from<E, C>(error: E, context: C) -> ErrorChain 
    where E: Error + Send + Sync + 'static,
        C: Display + Sync + Send + 'static {
            return ErrorChain { context: Box::new(context), cause: Some(Box::new(error)) }
        }
}

impl Display for ErrorChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context)?;
        if let Some(cause) = &self.cause {
            write!(f, "\n\\ \\ \\\n{}", cause)?;
        }
        Ok(())
    }
}

impl Debug for ErrorChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.context)?;
        if let Some(cause) = &self.cause {
            write!(f, "\n\\ \\ \\\n{:?}", cause)?;
        }
        Ok(())
    }
}

impl Error for ErrorChain {
    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     return self.cause;

    // }
}