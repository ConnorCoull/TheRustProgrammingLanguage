struct Date
{
    day: u8,
    month: u8,
    year: u32
}

fn get_date_from_string(s: &str) -> Date
{
    let units: Vec<&str> = s.split("/").collect();

    let day: u8 = units[0].parse().expect("Invalid day");
    let month: u8 = units[1].parse().expect("Invalid month");
    let year: u32 = units[2].parse().expect("Invalid year");

    Date
    {
        day:day,
        month:month,
        year:year
    }
}

fn older_student(date1: &Date, date2: &Date) -> bool{
    if date1.year < date2.year{
        return true;
    } else if date1.year > date2.year {
        return false;
    }

    if date1.month < date2.month{
        return true;
    } else if date1.month > date2.month {
        return false;
    }

    if date1.day < date2.day{
        return true;
    } else if date1.day > date2.day {
        return false;
    }

    false
}

struct Student 
{
    is_current_student: bool,
    firstname: String,
    surname: String,
    dob: Date,
    student_id: String

}

fn main() {
    let student1 = Student 
    {
        is_current_student:true,
        firstname:String::from("Connor"),
        surname:String::from("Coull"),
        dob:{get_date_from_string("18/04/2001")},
        student_id:String::from("2470724C")
    };

    let student2 = Student 
    {
        is_current_student:true,
        firstname:String::from("Lauren"),
        surname:String::from("Breckenridge"),
        dob:{get_date_from_string("27/09/2002")},
        student_id:String::from("2588819B")
    };

    let older_one: bool = {older_student(&student1.dob, &student2.dob)};

    if older_one{
        println!("{} {} is older!", student1.firstname, student1.surname)
    } else {
        println!("{} {} is older!", student2.firstname, student2.surname)
    }

}
