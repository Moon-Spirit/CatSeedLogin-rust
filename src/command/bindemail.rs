use pumpkin::{
    command::{ 
        args::ConsumedArgs, dispatcher::CommandError,
        CommandExecutor, CommandResult, CommandSender,
    },
    server::Server,
};

pub struct Bindemail; 

impl CommandExecutor for Bindemail {
    fn execute<'a>(
        &'a self,
        _sender: &'a CommandSender,
        _: &'a Server,
        _: &'a ConsumedArgs<'a>,
    ) -> CommandResult<'a> {
        Box::pin(async move {
            println!("邮箱绑定成功");
            Ok(())
        })
    }
}