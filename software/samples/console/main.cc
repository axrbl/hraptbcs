#include <service/console>


Console:Command!(cmd, "example console command")
{
    Console:println!("hello!");

    return 0;
}
