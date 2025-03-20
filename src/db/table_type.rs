use serde::{Deserialize, Serialize};

use crate::models::{
    address::Address, client::Client, contract::Contract, employee::Employee, payment::Payment,
    project::Project, task::Task, technology::Technology,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TableType {
    Technology(Technology),
    Task(Task),
    Project(Project),
    Payment(Payment),
    Employee(Employee),
    Contract(Contract),
    Client(Client),
    Address(Address),
}

pub trait GetParams {
    fn get_params(&self) -> mysql_async::Params;
}

impl GetParams for TableType {
    fn get_params(&self) -> mysql_async::Params {
        match self {
            TableType::Technology(tech) => mysql_async::Params::Positional(vec![
                tech.name.clone().into(),
                tech.description.clone().into(),
            ]),
            TableType::Task(task) => mysql_async::Params::Positional(vec![
                task.name.clone().into(),
                task.description.clone().into(),
                task.start_date.to_string().into(),
                task.end_date.to_string().into(),
                task.status.clone().into(),
            ]),
            TableType::Project(proj) => mysql_async::Params::Positional(vec![
                proj.name.clone().into(),
                proj.description.clone().into(),
                proj.start_date.to_string().into(),
                proj.end_date.to_string().into(),
                proj.status.clone().into(),
            ]),
            TableType::Payment(pay) => mysql_async::Params::Positional(vec![
                pay.amount.to_string().into(),
                pay.payment_due_date.to_string().into(),
                pay.method.clone().into(),
            ]),
            TableType::Employee(emp) => mysql_async::Params::Positional(vec![
                emp.first_name.clone().into(),
                emp.last_name.clone().into(),
                emp.email.clone().into(),
                emp.phone_number.clone().into(),
                emp.position.clone().into(),
                emp.contract_date.to_string().into(),
            ]),
            TableType::Contract(cont) => mysql_async::Params::Positional(vec![
                cont.type_of_contract.clone().into(),
                cont.start_date.to_string().into(),
                cont.end_date.to_string().into(),
                cont.salary.to_string().into(),
            ]),
            TableType::Client(cli) => mysql_async::Params::Positional(vec![
                cli.first_name.clone().into(),
                cli.last_name.clone().into(),
                cli.email.clone().into(),
                cli.phone_number.clone().into(),
            ]),
            TableType::Address(addr) => mysql_async::Params::Positional(vec![
                addr.city.clone().into(),
                addr.street.clone().into(),
                addr.street_number.clone().into(),
                addr.postal_code.clone().into(),
            ]),
        }
    }
}
