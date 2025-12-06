use pumpkin::{
    command::{ 
        args::ConsumedArgs, dispatcher::CommandError,
        CommandExecutor, CommandResult, CommandSender,
    },
    server::Server,
};

pub struct Changepassword; 

impl CommandExecutor for Changepassword {
    fn execute<'a>(
        &'a self,
        _sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            println!("密码修改成功");
            Ok(())
        })
    }
}