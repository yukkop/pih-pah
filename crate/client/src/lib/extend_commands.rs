#[macro_export]
macro_rules! extend_commands {
  ($command_name:ident($( $arg:ident: $arg_type:ty ),*), $command_fn:expr) => {
    #[allow(non_camel_case_types)]
    pub trait $command_name<'w, 's> {
      fn $command_name<'a>(
        &'a mut self,
        $($arg: $arg_type),*
      ) -> EntityCommands<'w, 's, 'a>;
    }

    impl<'w, 's> $command_name<'w, 's> for Commands<'w, 's> {
      fn $command_name<'a>(
        &'a mut self,
        $($arg: $arg_type),*
      ) -> EntityCommands<'w, 's, 'a> {
        let entity = self.spawn_empty();
        let entity_id = entity.id();

        self.add(move |world: &mut World| {
          #[allow(clippy::redundant_closure_call)]
          $command_fn(world, entity_id, $($arg),*);
        });

        self.entity(entity_id)
      }
    }
  };
}
