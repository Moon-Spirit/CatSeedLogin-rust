use pumpkin::{
    command::{ 
        args::ConsumedArgs, dispatcher::CommandError,
        CommandExecutor, CommandResult, CommandSender,
    },
    server::Server,
};

pub struct Login; 

impl CommandExecutor for Login {
    fn execute<'a>(
        &'a self,
        _sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            println!("登录成功");
            Ok(())
        })
    }
}