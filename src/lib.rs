extern crate console;
extern crate failure;
pub mod builder;
pub mod inputs;

use console::Term;
use failure::Error;
use inputs::{Input, LineInput};
use std::convert::Into;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Clim<T>
where
    T: Display + Eq + Into<String> + Clone + Debug,
{
    menu_options: Vec<MenuOption<T>>,
    pub title: String,
}

impl<T> Clim<T>
where
    T: Display + Eq + Into<String> + Clone,
{
    pub fn new<U: Into<Vec<MenuOption<T>>>>(menu_options: U, title: String) -> Clim<T> {
        Clim {
            menu_options: menu_options.into(),
            title,
        }
    }

    pub fn init(self) -> Result<(), Error> {
        let term = Term::stderr();
        term.write_line(&format!("{}", &self.title))?;

        loop {
            for menu_option in &self.menu_options {
                term.write_line(&format!("{} {}", menu_option.key, menu_option.description))?;
            }

            let mut line = LineInput::new(&term);
            line.get_from_terminal()?;

            match &self
                .menu_options
                .iter()
                .find(|&input| input.key.clone().into() == line.input)
            {
                Some(input) => {
                    (input.on_select)();

                    if input.is_exit {
                        break;
                    }
                }
                None => continue,
            };
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct MenuOption<T>
where
    T: Display + Eq + Into<String> + Clone,
{
    key: T,
    description: String,
    on_select: Rc<Fn()>,
    is_exit: bool,
}

impl<T> MenuOption<T>
where
    T: Display + Eq + Into<String> + Clone,
{
    fn new<U: Into<T>>(
        key: U,
        description: &str,
        on_select: Rc<Fn()>,
        is_exit: bool,
    ) -> MenuOption<T> {
        MenuOption {
            key: key.into(),
            description: description.to_owned(),
            on_select: on_select.clone(),
            is_exit,
        }
    }
}

impl<T> Debug for MenuOption<T>
where T: Into<String> + Display + Clone + Eq + Debug
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, " Menu Option {{ key: {}, description: {}, is_exit: {} }}", self.key, self.description, self.is_exit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_menu_option() {
        let menu_option = MenuOption {
            key: "1".to_string(),
            description: "foo bar baz".to_string(),
            on_select: Rc::new(|| {
                println!("yeeee");
            }),
            is_exit: false,
        };

        let clim = Clim::new(vec![(menu_option)], "Welcome To Clim".to_owned());

        (clim.menu_options.get(0).unwrap().on_select)();
    }

    #[test]
    fn clim_init() {
        let menu_option = vec![
            MenuOption {
                key: "1".to_string(),
                description: "foo bar baz".to_string(),
                on_select: Rc::new(|| {
                    println!("yeeee");
                }),
                is_exit: false,
            },
            MenuOption {
                key: "2".to_string(),
                description: "exit".to_string(),
                on_select: Rc::new(|| {
                    println!("exiting now");
                }),
                is_exit: true,
            },
        ];

        let _ = Clim::new(menu_option, "Welcome To Clim".to_owned()).init();
    }
}
