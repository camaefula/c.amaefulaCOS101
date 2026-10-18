fn main() {


    println!("Public Server APS");

    let mut profession = String::new();
    println!("Enter Profession (Office Admin, Academic, Lawyer, Teacher):");
    std::io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession =profession.trim();

    let mut experience = String::new();
    println!("Enter years of experience: ");
    std::io::stdin().read_line(&mut experience).expect("Failed to read input");
    let years:usize = experience.trim().parse().expect("Invalid input");
    

    let aps_level = match years {
        1..=2 => "APS 1-2",
        3..=5 => "APS 3-5",
        5..=8 => "APS 5-8",
        8..=10 => "EL1 8-10",
        10..13 => "EL2 10-13",
         _=> "SES",
    };

    let position = match profession {
            "Office Admin" => vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"],
            "Academics" => vec!["-","Research Assistant","PHD Candidate","Post-Doc Research","Senior Lecturer","Dean"],
            "Lawyer" => vec!["Paralegal","Juior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"],
            "Teacher" => vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"], 
            _ => vec!["Unknown","Unknown","Unknown","Unknown","Unknown","Unknown"],
    };
    let index = match aps_level {
      "APS 1-2" => 0,
      "APS 3-5" =>1,
      "APS 5-8" =>2,
       "EL1 8-10" =>3,
       "EL2 10-13" =>4,
       _ => 5,
    };

    let title = position[index];

    println!("\nStaff level verification");
    println!("Profession: {}",profession);
    println!("Years of experience: {}",years);
    println!("Calculated APS level: {}",aps_level);
    println!("Official title: {}",title);

}