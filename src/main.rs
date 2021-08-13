use crate::kk_field::Field;
use std::time::Instant;
use std::io;
//use std::pin::Pin;



mod kk_cell;
mod kk_field;
mod kk_inputs;


fn main() {


    println!("Enter filename of KenKen: ");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Could not read from standard input");
    let now = Instant::now();
    let mut f = Field::new(None);
    if file_name.starts_with("Dim") {
        f.initialize_from_definition(
        &kk_inputs::definition_inline(file_name.trim())
        ).expect("Init from inline definition failed");
    } else {
        f.initialize_from_definition(
        &kk_inputs::definition_from_file(file_name.trim())
        ).expect("Init from external definition failed");
    }

    println!("Start -\n{}",f);
    //println!("{:?}", f);

    let solution = kenken_solve(1,f);
    match solution {
        Some(sol) => println!("Solution -\n{}",sol),
        None => println!("Error"),
    }
    let dur=now.elapsed().as_millis();
    println!("Duration : {}:{}:{}.{}",dur/3600000,dur/60000 % 60,dur/1000 % 60,dur % 1000 );

}

fn kenken_solve(iteration: i32, field: Field) -> Option<Field> {

    //println!("{} -\n{}",iteration, field);
    let (count, temp_field, opt) = field.get_new_valid_field();
    //println!("{:?}{:?}{:?}", count,temp_field,opt);
    if count ==0 {
        // if count is zero recursion ends
        // if field is None there was an error ==> wait that threat is killed
        // otherwise field contains the found solution
             return temp_field;

        };
    // new iteration

    let option = opt.unwrap();
    let mut current_option:usize = 0;


    let mut new_field: Field = temp_field.unwrap();
    while new_field.apply_option_to_field(&option,current_option) {

        current_option +=1;
        if let Some(field)=kenken_solve(iteration+1, new_field.clone()) {
            return Some(field);
        };
    };


    None

}
