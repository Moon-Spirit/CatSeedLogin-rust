use pumpkin::{
    command::{ 
        args::ConsumedArgs, dispatcher::CommandError,
        CommandExecutor, CommandResult, CommandSender,
    },
    server::Server,
};

pub struct Register; 

impl CommandExecutor for Register {
    fn execute<'a>(
        &'a self,
        _sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            println!("注册成功");
            Ok(())
        })
    }
}