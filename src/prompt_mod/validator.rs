pub type Validator<T> = Box<dyn Fn(&T) -> bool + 'static>;
