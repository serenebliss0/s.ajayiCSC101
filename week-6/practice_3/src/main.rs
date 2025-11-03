fn main()
{
    let name1 = "Semire Ajayi";
    println!("My name is {}", name1);

    //find and replace functionality
    let name2 = name1.replace("Semire", "Serenity");
    println!("You can also call me {} ", name2);
    let faculty = "Faculty of Science and Technology";

    //find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}", school);
}