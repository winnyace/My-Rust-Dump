fn main()
{
    for num in 1..101
    {
        if num % 15 == 0 
        {
            println!("fizzbuzz");
        }
        else if num % 3 == 0
        {
            println!("fizz");
        }
        else if num % 5 == 0
        {
            println!("buzz");
        }
        else
        {
            println!("{}", num);
        }
    }
}
