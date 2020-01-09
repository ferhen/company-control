use std::collections::HashMap;
use std::io;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ADD_EMPLOYEE_CMD: Regex = Regex::new(r"Add \w* to \w*$").unwrap();
    static ref GET_DEPARTMENT_EMPLOYEES_CMD: Regex = Regex::new(r"Show employees from \w*$").unwrap();
    static ref GET_ALL_EMPLOYEES_CMD: Regex = Regex::new(r"Show all employees").unwrap();
}

#[derive(Debug)]
enum InputType {
    AddEmployee {
        employee: String,
        department: String
    },
    GetEmployeesByDepartment(String),
    GetAllEmployees
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match get_input_type(&input.trim()) {
                    Ok(InputType::AddEmployee {employee, department}) => add_employee(&mut company, employee, department),
                    Ok(InputType::GetEmployeesByDepartment(department)) => show_employees_from_department(&company, &department),
                    Ok(InputType::GetAllEmployees) => show_all_employees(&company),
                    Err(e) => println!("Erro: {}", e)
                }
            },
            Err(error) => println!("Erro: {}", error)
        };
    }
}

fn get_input_type(input_string: &str) -> Result<InputType, io::Error> {
    let words: Vec<&str> = input_string.split_whitespace().collect();
    if ADD_EMPLOYEE_CMD.is_match(input_string) {
        return Ok(InputType::AddEmployee {
            employee: String::from(*words.get(1).unwrap()),
            department: String::from(*words.get(3).unwrap())
        });
    }
    else if GET_DEPARTMENT_EMPLOYEES_CMD.is_match(input_string) {
        return Ok(InputType::GetEmployeesByDepartment(String::from(*words.get(3).unwrap())));
    }
    else if GET_ALL_EMPLOYEES_CMD.is_match(input_string) {
        return Ok(InputType::GetAllEmployees);
    }
    else {
        return Err(io::Error::new(io::ErrorKind::Other, "Comando Inválido!"));
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, employee: String, department: String) -> () {
    println!("Funcionário {} adicionado ao departamento {}!", employee, department);
    match company.get_mut(&department) {
        Some(existing_department) => existing_department.push(employee),
        None => {
            let mut new_company: Vec<String> = Vec::new();
            new_company.push(employee);
            company.insert(department, new_company);
        }
    }
}

fn show_employees_from_department(company: &HashMap<String, Vec<String>>, department: &String) -> () {
    match company.get(department) {
        Some(existing_department) => println!("{:#?}", existing_department),
        None => println!("Nenhum funcionário registrado no departamento {}", &department)
    }
}

fn show_all_employees(company: &HashMap<String, Vec<String>>) -> () {
    println!("{:#?}", company);
}