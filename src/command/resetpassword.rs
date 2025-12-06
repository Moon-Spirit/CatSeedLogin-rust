use pumpkin::{
    command::{ 
        args::ConsumedArgs, dispatcher::CommandError,
        CommandExecutor, CommandResult, CommandSender,
    },
    server::Server,
};

pub struct Resetpassword; 

impl CommandExecutor for Resetpassword {
    fn execute<'a>(
        &'a self,
        _sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            println!("密码重置成功");
            Ok(())
        })
    }
}