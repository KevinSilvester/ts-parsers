#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod cli {
    use clap::builder::{
        styling::{AnsiColor, Effects},
        Styles,
    };
    use crate::subcommands::{
        Backups, Compile, Install, Lock, Subcommand, Uninstall, Unlock, Update,
    };
    fn style() -> Styles {
        Styles::default()
            .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
            .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
            .literal(AnsiColor::Green.on_default() | Effects::BOLD)
            .valid(AnsiColor::Green.on_default() | Effects::BOLD)
            .placeholder(AnsiColor::Green.on_default() | Effects::BOLD)
            .error(AnsiColor::Red.on_default() | Effects::BOLD)
    }
    #[clap(version, about, styles = style())]
    pub enum Cli {
        /// Create/restore backups of installed tree-sitter parsers
        #[clap(name = "backups")]
        Backups(Backups),
        /// Display changelog for nvim-treesitter-parsers
        #[clap(name = "changelog")]
        Changelog,
        /// Clean the download cache,backups and dangling compile artefacts
        #[clap(name = "clean")]
        Clean,
        /// Compile tree-sitter parsers with zig or clang to a target directory
        #[clap(name = "compile")]
        Compile(Compile),
        /// Compile/Download tree-sitter parsers and install them
        #[clap(name = "install")]
        Install(Install),
        /// Lock the current state of installed tree-sitter parsers
        #[clap(name = "lock")]
        Lock(Lock),
        /// Self update
        #[clap(name = "self-update")]
        SelfUpdate,
        /// List all tree-sitter parsers (installed and available)
        #[clap(name = "parsers")]
        Parsers,
        /// Uninstall tree-sitter parsers
        #[clap(name = "uninstall")]
        Uninstall(Uninstall),
        /// Update installed tree-sitter parsers
        #[clap(name = "update")]
        Update(Update),
        /// Unlock the current state of installed tree-sitter parsers
        #[clap(name = "unlock")]
        Unlock(Unlock),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Cli {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Cli::Backups(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Backups",
                        &__self_0,
                    )
                }
                Cli::Changelog => ::core::fmt::Formatter::write_str(f, "Changelog"),
                Cli::Clean => ::core::fmt::Formatter::write_str(f, "Clean"),
                Cli::Compile(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Compile",
                        &__self_0,
                    )
                }
                Cli::Install(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Install",
                        &__self_0,
                    )
                }
                Cli::Lock(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Lock",
                        &__self_0,
                    )
                }
                Cli::SelfUpdate => ::core::fmt::Formatter::write_str(f, "SelfUpdate"),
                Cli::Parsers => ::core::fmt::Formatter::write_str(f, "Parsers"),
                Cli::Uninstall(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Uninstall",
                        &__self_0,
                    )
                }
                Cli::Update(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Update",
                        &__self_0,
                    )
                }
                Cli::Unlock(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Unlock",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl clap::Parser for Cli {}
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::CommandFactory for Cli {
        fn command<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("ts-parsers")
                .subcommand_required(true)
                .arg_required_else_help(true);
            <Self as clap::Subcommand>::augment_subcommands(__clap_app)
        }
        fn command_for_update<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("ts-parsers");
            <Self as clap::Subcommand>::augment_subcommands_for_update(__clap_app)
                .subcommand_required(false)
                .arg_required_else_help(false)
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for Cli {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
                .remove_subcommand()
            {
                let __clap_arg_matches = &mut __clap_arg_sub_matches;
                if __clap_name == "backups" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(
                        Self::Backups(
                            <Backups as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                if __clap_name == "changelog" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Changelog);
                }
                if __clap_name == "clean" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Clean);
                }
                if __clap_name == "compile" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(
                        Self::Compile(
                            <Compile as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                if __clap_name == "install" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(
                        Self::Install(
                            <Install as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                if __clap_name == "lock" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(
                        Self::Lock(
                            <Lock as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                if __clap_name == "self-update" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::SelfUpdate);
                }
                if __clap_name == "parsers" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(Self::Parsers);
                }
                if __clap_name == "uninstall" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(
                        Self::Uninstall(
                            <Uninstall as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                if __clap_name == "update" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(
                        Self::Update(
                            <Update as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                if __clap_name == "unlock" && !__clap_arg_matches.contains_id("") {
                    return ::std::result::Result::Ok(
                        Self::Unlock(
                            <Unlock as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?,
                        ),
                    );
                }
                ::std::result::Result::Err(
                    clap::Error::raw(
                        clap::error::ErrorKind::InvalidSubcommand,
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "The subcommand \'{0}\' wasn\'t recognized",
                                    __clap_name,
                                ),
                            );
                            res
                        },
                    ),
                )
            } else {
                ::std::result::Result::Err(
                    clap::Error::raw(
                        clap::error::ErrorKind::MissingSubcommand,
                        "A subcommand is required but one was not provided.",
                    ),
                )
            }
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut<'b>(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                match self {
                    Self::Backups(ref mut __clap_arg) if "backups" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::Changelog if "changelog" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Clean if "clean" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Compile(ref mut __clap_arg) if "compile" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::Install(ref mut __clap_arg) if "install" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::Lock(ref mut __clap_arg) if "lock" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::SelfUpdate if "self-update" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Parsers if "parsers" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {}
                    }
                    Self::Uninstall(
                        ref mut __clap_arg,
                    ) if "uninstall" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::Update(ref mut __clap_arg) if "update" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    Self::Unlock(ref mut __clap_arg) if "unlock" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        clap::FromArgMatches::update_from_arg_matches_mut(
                            __clap_arg,
                            __clap_arg_matches,
                        )?
                    }
                    s => {
                        *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?;
                    }
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::Subcommand for Cli {
        fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("backups");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Backups as clap::Args>::augment_args(__clap_subcommand)
                    };
                    __clap_subcommand
                        .about("Create/restore backups of installed tree-sitter parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("changelog");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Display changelog for nvim-treesitter-parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("clean");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Clean the download cache,backups and dangling compile artefacts",
                        )
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("compile");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Compile as clap::Args>::augment_args(__clap_subcommand)
                    };
                    __clap_subcommand
                        .about(
                            "Compile tree-sitter parsers with zig or clang to a target directory",
                        )
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("install");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Install as clap::Args>::augment_args(__clap_subcommand)
                    };
                    __clap_subcommand
                        .about("Compile/Download tree-sitter parsers and install them")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("lock");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Lock as clap::Args>::augment_args(__clap_subcommand)
                    };
                    __clap_subcommand
                        .about("Lock the current state of installed tree-sitter parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("self-update");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand.about("Self update").long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("parsers");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("List all tree-sitter parsers (installed and available)")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("uninstall");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Uninstall as clap::Args>::augment_args(__clap_subcommand)
                    };
                    __clap_subcommand
                        .about("Uninstall tree-sitter parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("update");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Update as clap::Args>::augment_args(__clap_subcommand)
                    };
                    __clap_subcommand
                        .about("Update installed tree-sitter parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("unlock");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Unlock as clap::Args>::augment_args(__clap_subcommand)
                    };
                    __clap_subcommand
                        .about(
                            "Unlock the current state of installed tree-sitter parsers",
                        )
                        .long_about(None)
                });
            __clap_app
                .version("0.0.0")
                .about("A tool to manage tree-sitter parsers used by nvim-treesitter")
                .styles(style())
        }
        fn augment_subcommands_for_update<'b>(
            __clap_app: clap::Command,
        ) -> clap::Command {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("backups");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Backups as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                        .about("Create/restore backups of installed tree-sitter parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("changelog");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("Display changelog for nvim-treesitter-parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("clean");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about(
                            "Clean the download cache,backups and dangling compile artefacts",
                        )
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("compile");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Compile as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                        .about(
                            "Compile tree-sitter parsers with zig or clang to a target directory",
                        )
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("install");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Install as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                        .about("Compile/Download tree-sitter parsers and install them")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("lock");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Lock as clap::Args>::augment_args_for_update(__clap_subcommand)
                    };
                    __clap_subcommand
                        .about("Lock the current state of installed tree-sitter parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("self-update");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand.about("Self update").long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("parsers");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = __clap_subcommand;
                    __clap_subcommand
                        .about("List all tree-sitter parsers (installed and available)")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("uninstall");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Uninstall as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                        .about("Uninstall tree-sitter parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("update");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Update as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                        .about("Update installed tree-sitter parsers")
                        .long_about(None)
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("unlock");
                    let __clap_subcommand = __clap_subcommand;
                    let __clap_subcommand = {
                        <Unlock as clap::Args>::augment_args_for_update(
                            __clap_subcommand,
                        )
                    };
                    __clap_subcommand
                        .about(
                            "Unlock the current state of installed tree-sitter parsers",
                        )
                        .long_about(None)
                });
            __clap_app
                .version("0.0.0")
                .about("A tool to manage tree-sitter parsers used by nvim-treesitter")
                .styles(style())
        }
        fn has_subcommand(__clap_name: &str) -> bool {
            if "backups" == __clap_name {
                return true;
            }
            if "changelog" == __clap_name {
                return true;
            }
            if "clean" == __clap_name {
                return true;
            }
            if "compile" == __clap_name {
                return true;
            }
            if "install" == __clap_name {
                return true;
            }
            if "lock" == __clap_name {
                return true;
            }
            if "self-update" == __clap_name {
                return true;
            }
            if "parsers" == __clap_name {
                return true;
            }
            if "uninstall" == __clap_name {
                return true;
            }
            if "update" == __clap_name {
                return true;
            }
            if "unlock" == __clap_name {
                return true;
            }
            false
        }
    }
    impl Cli {
        pub async fn run(&self) -> anyhow::Result<()> {
            match self {
                Self::Compile(cmd) => cmd.run().await?,
                Self::Install(cmd) => cmd.run().await?,
                Self::Update(cmd) => cmd.run().await?,
                Self::Uninstall(cmd) => cmd.run().await?,
                Self::Lock(cmd) => cmd.run().await?,
                Self::Unlock(cmd) => cmd.run().await?,
                Self::Backups(cmd) => cmd.run().await?,
                Self::Changelog => ::core::panicking::panic("not yet implemented"),
                Self::Clean => ::core::panicking::panic("not yet implemented"),
                Self::SelfUpdate => ::core::panicking::panic("not yet implemented"),
                Self::Parsers => ::core::panicking::panic("not yet implemented"),
            }
            Ok(())
        }
    }
}
mod compiler {
    mod cc {
        use std::path::Path;
        use crate::utils::command;
        use super::{zig::ZigTargets, Compiler};
        pub struct CC<'a> {
            command: &'a str,
            base_args: &'a [&'a str],
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for CC<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CC",
                    "command",
                    &self.command,
                    "base_args",
                    &&self.base_args,
                )
            }
        }
        impl<'a> CC<'a> {
            pub const GCC: &'static str = "gcc";
            pub const CLANG: &'static str = "clang";
            pub fn new(command: &'a str) -> Self {
                #[cfg(target_os = "linux")]
                let base_args = &["-o", "out.so", "-Os", "-shared", "-fPIC"];
                Self { command, base_args }
            }
            fn build_args(&self, files: &'a [&'a str]) -> Vec<&'a str> {
                let mut args = ::alloc::vec::Vec::new();
                args.extend(self.base_args);
                if files
                    .iter()
                    .any(|file| {
                        file.ends_with(".cc") || file.ends_with(".cpp")
                            || file.ends_with(".cxx")
                    })
                {
                    args.push("-lstdc++");
                }
                args.push("-Isrc");
                args.extend(files);
                args
            }
        }
        impl Compiler for CC<'_> {
            fn get_name(&self) -> &str {
                "clang"
            }
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn compile<'life0, 'life1, 'life2, 'life3, 'life4, 'async_trait>(
                &'life0 self,
                files: &'life1 [&'life2 str],
                cwd: &'life3 Path,
                _: &'life4 Option<ZigTargets>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                'life3: 'async_trait,
                'life4: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        command::check_exists("clang")?;
                        let args = __self.build_args(files);
                        match command::run(__self.command, &args, Some(cwd)).await? {
                            true => Ok(()),
                            false => {
                                return ::anyhow::__private::Err({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("Failed to compile files"),
                                    );
                                    error
                                });
                            }
                        }
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    mod zig {
        use std::path::Path;
        use crate::utils::command;
        use super::Compiler;
        pub enum ZigTargets {
            #[clap(name = "x86_64-linux")]
            X86_64Linux,
            #[clap(name = "aarch64-linux")]
            Aarch64Linux,
            #[clap(name = "x86_64-macos")]
            X86_64MacOS,
            #[clap(name = "aarch64-macos")]
            Aarch64MacOS,
            #[clap(name = "x86_64-windows")]
            X86_64Windows,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ZigTargets {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ZigTargets::X86_64Linux => "X86_64Linux",
                        ZigTargets::Aarch64Linux => "Aarch64Linux",
                        ZigTargets::X86_64MacOS => "X86_64MacOS",
                        ZigTargets::Aarch64MacOS => "Aarch64MacOS",
                        ZigTargets::X86_64Windows => "X86_64Windows",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ZigTargets {
            #[inline]
            fn clone(&self) -> ZigTargets {
                match self {
                    ZigTargets::X86_64Linux => ZigTargets::X86_64Linux,
                    ZigTargets::Aarch64Linux => ZigTargets::Aarch64Linux,
                    ZigTargets::X86_64MacOS => ZigTargets::X86_64MacOS,
                    ZigTargets::Aarch64MacOS => ZigTargets::Aarch64MacOS,
                    ZigTargets::X86_64Windows => ZigTargets::X86_64Windows,
                }
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::ValueEnum for ZigTargets {
            fn value_variants<'a>() -> &'a [Self] {
                &[
                    Self::X86_64Linux,
                    Self::Aarch64Linux,
                    Self::X86_64MacOS,
                    Self::Aarch64MacOS,
                    Self::X86_64Windows,
                ]
            }
            fn to_possible_value<'a>(
                &self,
            ) -> ::std::option::Option<clap::builder::PossibleValue> {
                match self {
                    Self::X86_64Linux => {
                        Some({ clap::builder::PossibleValue::new("x86_64-linux") })
                    }
                    Self::Aarch64Linux => {
                        Some({ clap::builder::PossibleValue::new("aarch64-linux") })
                    }
                    Self::X86_64MacOS => {
                        Some({ clap::builder::PossibleValue::new("x86_64-macos") })
                    }
                    Self::Aarch64MacOS => {
                        Some({ clap::builder::PossibleValue::new("aarch64-macos") })
                    }
                    Self::X86_64Windows => {
                        Some({ clap::builder::PossibleValue::new("x86_64-windows") })
                    }
                    _ => None,
                }
            }
        }
        pub struct Zig<'a> {
            command: &'a str,
            base_args: &'a [&'a str],
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for Zig<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Zig",
                    "command",
                    &self.command,
                    "base_args",
                    &&self.base_args,
                )
            }
        }
        impl<'a> Zig<'a> {
            pub fn new() -> Self {
                Self {
                    command: "zig",
                    base_args: &["c++", "-o", "out.so", "-lc", "-shared", "-Os"],
                }
            }
            fn build_args(
                &self,
                files: &'a [&'a str],
                target: &Option<ZigTargets>,
            ) -> Vec<&'a str> {
                let mut args = ::alloc::vec::Vec::new();
                args.extend(self.base_args);
                args.push("-Isrc");
                args.extend(files);
                match target {
                    Some(ZigTargets::X86_64Linux) => {
                        args.push("-target");
                        args.push("x86_64-linux");
                    }
                    Some(ZigTargets::Aarch64Linux) => {
                        args.push("-target");
                        args.push("aarch64-linux");
                    }
                    Some(ZigTargets::X86_64MacOS) => {
                        args.push("-target");
                        args.push("x86_64-macos");
                    }
                    Some(ZigTargets::Aarch64MacOS) => {
                        args.push("-target");
                        args.push("aarch64-macos");
                    }
                    Some(ZigTargets::X86_64Windows) => {
                        args.push("-target");
                        args.push("x86_64-windows");
                    }
                    None => {}
                }
                args
            }
        }
        impl Compiler for Zig<'_> {
            fn get_name(&self) -> &str {
                "zig"
            }
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn compile<'life0, 'life1, 'life2, 'life3, 'life4, 'async_trait>(
                &'life0 self,
                files: &'life1 [&'life2 str],
                cwd: &'life3 Path,
                target: &'life4 Option<ZigTargets>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                'life3: 'async_trait,
                'life4: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        command::check_exists("zig")?;
                        let args = __self.build_args(files, target);
                        match command::run(__self.command, &args, Some(cwd)).await? {
                            true => Ok(()),
                            false => {
                                return ::anyhow::__private::Err({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("Failed to compile files"),
                                    );
                                    error
                                });
                            }
                        }
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    use std::path::Path;
    use self::cc::CC;
    use self::zig::Zig;
    pub use self::zig::ZigTargets;
    pub trait Compiler: Send + Sync {
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn compile<'life0, 'life1, 'life2, 'life3, 'life4, 'async_trait>(
            &'life0 self,
            files: &'life1 [&'life2 str],
            cwd: &'life3 Path,
            target: &'life4 Option<ZigTargets>,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<()>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            'life3: 'async_trait,
            'life4: 'async_trait,
            Self: 'async_trait;
        fn get_name(&self) -> &str;
    }
    pub enum CompilerOption {
        #[default]
        Clang,
        Gcc,
        Zig,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CompilerOption {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    CompilerOption::Clang => "Clang",
                    CompilerOption::Gcc => "Gcc",
                    CompilerOption::Zig => "Zig",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for CompilerOption {
        #[inline]
        fn default() -> CompilerOption {
            Self::Clang
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CompilerOption {
        #[inline]
        fn clone(&self) -> CompilerOption {
            match self {
                CompilerOption::Clang => CompilerOption::Clang,
                CompilerOption::Gcc => CompilerOption::Gcc,
                CompilerOption::Zig => CompilerOption::Zig,
            }
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::ValueEnum for CompilerOption {
        fn value_variants<'a>() -> &'a [Self] {
            &[Self::Clang, Self::Gcc, Self::Zig]
        }
        fn to_possible_value<'a>(
            &self,
        ) -> ::std::option::Option<clap::builder::PossibleValue> {
            match self {
                Self::Clang => Some({ clap::builder::PossibleValue::new("clang") }),
                Self::Gcc => Some({ clap::builder::PossibleValue::new("gcc") }),
                Self::Zig => Some({ clap::builder::PossibleValue::new("zig") }),
                _ => None,
            }
        }
    }
    pub fn select_compiler(compiler: &CompilerOption) -> Box<dyn Compiler> {
        match compiler {
            CompilerOption::Clang => Box::new(CC::new(CC::CLANG)),
            CompilerOption::Gcc => Box::new(CC::new(CC::GCC)),
            CompilerOption::Zig => Box::new(Zig::new()),
        }
    }
}
mod data {
    pub mod changelog {
        use chrono::{DateTime, Utc};
        use serde::{Deserialize, Serialize};
        pub struct Changes {
            pub added: Vec<String>,
            pub updated: Vec<String>,
            pub removed: Vec<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Changes {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Changes",
                    "added",
                    &self.added,
                    "updated",
                    &self.updated,
                    "removed",
                    &&self.removed,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Changes {
            #[inline]
            fn clone(&self) -> Changes {
                Changes {
                    added: ::core::clone::Clone::clone(&self.added),
                    updated: ::core::clone::Clone::clone(&self.updated),
                    removed: ::core::clone::Clone::clone(&self.removed),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Changes {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "added" => _serde::__private::Ok(__Field::__field0),
                                "updated" => _serde::__private::Ok(__Field::__field1),
                                "removed" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"added" => _serde::__private::Ok(__Field::__field0),
                                b"updated" => _serde::__private::Ok(__Field::__field1),
                                b"removed" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Changes>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Changes;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Changes",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                Vec<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Changes with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Vec<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Changes with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Vec<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Changes with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Changes {
                                added: __field0,
                                updated: __field1,
                                removed: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("added"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updated",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "removed",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("added")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("updated")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("removed")?
                                }
                            };
                            _serde::__private::Ok(Changes {
                                added: __field0,
                                updated: __field1,
                                removed: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "added",
                        "updated",
                        "removed",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Changes",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Changes>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Changes {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Changes",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "added",
                        &self.added,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "updated",
                        &self.updated,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "removed",
                        &self.removed,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct ChangeLogEntry {
            pub tag: String,
            pub url: String,
            pub date: DateTime<Utc>,
            pub changes: Changes,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ChangeLogEntry {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "ChangeLogEntry",
                    "tag",
                    &self.tag,
                    "url",
                    &self.url,
                    "date",
                    &self.date,
                    "changes",
                    &&self.changes,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ChangeLogEntry {
            #[inline]
            fn clone(&self) -> ChangeLogEntry {
                ChangeLogEntry {
                    tag: ::core::clone::Clone::clone(&self.tag),
                    url: ::core::clone::Clone::clone(&self.url),
                    date: ::core::clone::Clone::clone(&self.date),
                    changes: ::core::clone::Clone::clone(&self.changes),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ChangeLogEntry {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "tag" => _serde::__private::Ok(__Field::__field0),
                                "url" => _serde::__private::Ok(__Field::__field1),
                                "date" => _serde::__private::Ok(__Field::__field2),
                                "changes" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"tag" => _serde::__private::Ok(__Field::__field0),
                                b"url" => _serde::__private::Ok(__Field::__field1),
                                b"date" => _serde::__private::Ok(__Field::__field2),
                                b"changes" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ChangeLogEntry>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ChangeLogEntry;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ChangeLogEntry",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ChangeLogEntry with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ChangeLogEntry with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                DateTime<Utc>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ChangeLogEntry with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Changes,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct ChangeLogEntry with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ChangeLogEntry {
                                tag: __field0,
                                url: __field1,
                                date: __field2,
                                changes: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<DateTime<Utc>> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Changes> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("tag"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("date"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                DateTime<Utc>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "changes",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Changes>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("tag")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("date")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("changes")?
                                }
                            };
                            _serde::__private::Ok(ChangeLogEntry {
                                tag: __field0,
                                url: __field1,
                                date: __field2,
                                changes: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "tag",
                        "url",
                        "date",
                        "changes",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ChangeLogEntry",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ChangeLogEntry>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ChangeLogEntry {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ChangeLogEntry",
                        false as usize + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "tag",
                        &self.tag,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "date",
                        &self.date,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "changes",
                        &self.changes,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct ChangeLog {
            pub entries: Vec<ChangeLogEntry>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ChangeLog {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ChangeLog",
                    "entries",
                    &&self.entries,
                )
            }
        }
        impl ChangeLog {
            pub fn new() -> Self {
                Self { entries: Vec::new() }
            }
            pub async fn fetch_changelog(&mut self) -> anyhow::Result<()> {
                let url = "https://raw.githubusercontent.com/KevinSilvester/nvim-treesitter-parsers/master/CHANGELOG.min.json";
                let res = reqwest::get(url).await?;
                self.entries = res.json().await?;
                Ok(())
            }
            pub fn get_latest(&self) -> Option<&ChangeLogEntry> {
                self.entries.first()
            }
            pub fn get_latest_tag(&self) -> Option<String> {
                self.get_latest().map(|entry| entry.tag.clone())
            }
            pub fn check_entry(&self, tag: &Option<String>) -> anyhow::Result<()> {
                let tag = match tag {
                    Some(tag) => tag,
                    None => return Ok(()),
                };
                match self.entries.iter().any(|entry| &entry.tag == tag) {
                    true => Ok(()),
                    false => {
                        Err(
                            ::anyhow::Error::msg({
                                let res = ::alloc::fmt::format(
                                    format_args!("Tag {0} not found in changelog", tag),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
    }
    pub mod parsers {
        use ahash::AHashMap;
        use serde::{Deserialize, Serialize};
        use crate::utils::PATHS;
        pub struct ParserInfo {
            pub url: String,
            pub files: Vec<String>,
            pub location: Option<String>,
            pub revision: String,
            pub generate_from_grammar: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ParserInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ParserInfo",
                    "url",
                    &self.url,
                    "files",
                    &self.files,
                    "location",
                    &self.location,
                    "revision",
                    &self.revision,
                    "generate_from_grammar",
                    &&self.generate_from_grammar,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ParserInfo {
            #[inline]
            fn clone(&self) -> ParserInfo {
                ParserInfo {
                    url: ::core::clone::Clone::clone(&self.url),
                    files: ::core::clone::Clone::clone(&self.files),
                    location: ::core::clone::Clone::clone(&self.location),
                    revision: ::core::clone::Clone::clone(&self.revision),
                    generate_from_grammar: ::core::clone::Clone::clone(
                        &self.generate_from_grammar,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ParserInfo {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ParserInfo {
            #[inline]
            fn eq(&self, other: &ParserInfo) -> bool {
                self.url == other.url && self.files == other.files
                    && self.location == other.location && self.revision == other.revision
                    && self.generate_from_grammar == other.generate_from_grammar
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ParserInfo {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "url" => _serde::__private::Ok(__Field::__field0),
                                "files" => _serde::__private::Ok(__Field::__field1),
                                "location" => _serde::__private::Ok(__Field::__field2),
                                "revision" => _serde::__private::Ok(__Field::__field3),
                                "generate_from_grammar" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"url" => _serde::__private::Ok(__Field::__field0),
                                b"files" => _serde::__private::Ok(__Field::__field1),
                                b"location" => _serde::__private::Ok(__Field::__field2),
                                b"revision" => _serde::__private::Ok(__Field::__field3),
                                b"generate_from_grammar" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ParserInfo>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ParserInfo;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ParserInfo",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ParserInfo with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Vec<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ParserInfo with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ParserInfo with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct ParserInfo with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct ParserInfo with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ParserInfo {
                                url: __field0,
                                files: __field1,
                                location: __field2,
                                revision: __field3,
                                generate_from_grammar: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("files"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "revision",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "generate_from_grammar",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("files")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("revision")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field(
                                        "generate_from_grammar",
                                    )?
                                }
                            };
                            _serde::__private::Ok(ParserInfo {
                                url: __field0,
                                files: __field1,
                                location: __field2,
                                revision: __field3,
                                generate_from_grammar: __field4,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "url",
                        "files",
                        "location",
                        "revision",
                        "generate_from_grammar",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ParserInfo",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ParserInfo>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ParserInfo {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ParserInfo",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "files",
                        &self.files,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "revision",
                        &self.revision,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "generate_from_grammar",
                        &self.generate_from_grammar,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct Parsers {
            pub langs: Vec<String>,
            pub list: AHashMap<String, ParserInfo>,
            pub wanted: Option<Vec<String>>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Parsers {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Parsers",
                    "langs",
                    &self.langs,
                    "list",
                    &self.list,
                    "wanted",
                    &&self.wanted,
                )
            }
        }
        impl Parsers {
            pub fn new() -> anyhow::Result<Self> {
                let wanted_file = &PATHS.wanted_parsers;
                let wanted = match wanted_file.is_file() {
                    true => {
                        Some(
                            std::fs::read_to_string(wanted_file)?
                                .lines()
                                .map(str::trim)
                                .filter(|line| !line.is_empty())
                                .map(str::to_string)
                                .collect(),
                        )
                    }
                    false => None,
                };
                Ok(Self {
                    langs: Vec::new(),
                    list: AHashMap::new(),
                    wanted,
                })
            }
            pub async fn fetch_list(
                &mut self,
                tag: &Option<String>,
            ) -> anyhow::Result<()> {
                let url = match tag {
                    Some(tag) => {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "https://raw.githubusercontent.com/KevinSilvester/nvim-treesitter-parsers/{0}/parsers.min.json",
                                tag,
                            ),
                        );
                        res
                    }
                    None => {
                        "https://raw.githubusercontent.com/KevinSilvester/nvim-treesitter-parsers/master/parsers.min.json"
                            .to_string()
                    }
                };
                let res = reqwest::get(&url).await?;
                self.list = res.json().await?;
                self.langs = self.list.keys().cloned().collect();
                self.langs.sort();
                Ok(())
            }
            pub fn get_parser(&self, parser: &str) -> Option<&ParserInfo> {
                self.list.get(parser)
            }
            pub fn validate_parsers(&self, parsers: &[String]) -> anyhow::Result<()> {
                let invalid_parsers: Vec<_> = parsers
                    .iter()
                    .filter(|p| !self.list.contains_key(*p))
                    .collect();
                if !invalid_parsers.is_empty() {
                    return ::anyhow::__private::Err(
                        ::anyhow::Error::msg({
                            let res = ::alloc::fmt::format(
                                format_args!("Invalid parsers: {0:?}", invalid_parsers),
                            );
                            res
                        }),
                    );
                }
                Ok(())
            }
        }
    }
    pub mod state {
        use std::{
            collections::{BTreeMap, VecDeque},
            path::{Path, PathBuf},
        };
        use chrono::{DateTime, Utc};
        use serde::{Deserialize, Serialize};
        use crate::utils::PATHS;
        use super::parsers::ParserInfo;
        pub enum ParserInstallMethod {
            #[default]
            Compile,
            Download,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ParserInstallMethod {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ParserInstallMethod::Compile => "Compile",
                        ParserInstallMethod::Download => "Download",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ParserInstallMethod {
            #[inline]
            fn clone(&self) -> ParserInstallMethod {
                match self {
                    ParserInstallMethod::Compile => ParserInstallMethod::Compile,
                    ParserInstallMethod::Download => ParserInstallMethod::Download,
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ParserInstallMethod {
            #[inline]
            fn default() -> ParserInstallMethod {
                Self::Compile
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ParserInstallMethod {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ParserInstallMethod {
            #[inline]
            fn eq(&self, other: &ParserInstallMethod) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ParserInstallMethod {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        ParserInstallMethod::Compile => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "ParserInstallMethod",
                                0u32,
                                "Compile",
                            )
                        }
                        ParserInstallMethod::Download => {
                            _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "ParserInstallMethod",
                                1u32,
                                "Download",
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ParserInstallMethod {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
                                        ),
                                    )
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Compile" => _serde::__private::Ok(__Field::__field0),
                                "Download" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Compile" => _serde::__private::Ok(__Field::__field0),
                                b"Download" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ParserInstallMethod>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ParserInstallMethod;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum ParserInstallMethod",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(ParserInstallMethod::Compile)
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::de::VariantAccess::unit_variant(__variant)?;
                                    _serde::__private::Ok(ParserInstallMethod::Download)
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Compile", "Download"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "ParserInstallMethod",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                ParserInstallMethod,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::ValueEnum for ParserInstallMethod {
            fn value_variants<'a>() -> &'a [Self] {
                &[Self::Compile, Self::Download]
            }
            fn to_possible_value<'a>(
                &self,
            ) -> ::std::option::Option<clap::builder::PossibleValue> {
                match self {
                    Self::Compile => {
                        Some({ clap::builder::PossibleValue::new("compile") })
                    }
                    Self::Download => {
                        Some({ clap::builder::PossibleValue::new("download") })
                    }
                    _ => None,
                }
            }
        }
        pub struct ParserState {
            pub last_modified: DateTime<Utc>,
            pub revision: String,
            pub url: String,
            pub tag: String,
            pub locked: bool,
            pub install_method: ParserInstallMethod,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ParserState {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "last_modified",
                    "revision",
                    "url",
                    "tag",
                    "locked",
                    "install_method",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.last_modified,
                    &self.revision,
                    &self.url,
                    &self.tag,
                    &self.locked,
                    &&self.install_method,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ParserState",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ParserState {
            #[inline]
            fn clone(&self) -> ParserState {
                ParserState {
                    last_modified: ::core::clone::Clone::clone(&self.last_modified),
                    revision: ::core::clone::Clone::clone(&self.revision),
                    url: ::core::clone::Clone::clone(&self.url),
                    tag: ::core::clone::Clone::clone(&self.tag),
                    locked: ::core::clone::Clone::clone(&self.locked),
                    install_method: ::core::clone::Clone::clone(&self.install_method),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ParserState {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ParserState {
            #[inline]
            fn eq(&self, other: &ParserState) -> bool {
                self.last_modified == other.last_modified
                    && self.revision == other.revision && self.url == other.url
                    && self.tag == other.tag && self.locked == other.locked
                    && self.install_method == other.install_method
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ParserState {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ParserState",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "last_modified",
                        &self.last_modified,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "revision",
                        &self.revision,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "url",
                        &self.url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "tag",
                        &self.tag,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "locked",
                        &self.locked,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "install_method",
                        &self.install_method,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ParserState {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "last_modified" => _serde::__private::Ok(__Field::__field0),
                                "revision" => _serde::__private::Ok(__Field::__field1),
                                "url" => _serde::__private::Ok(__Field::__field2),
                                "tag" => _serde::__private::Ok(__Field::__field3),
                                "locked" => _serde::__private::Ok(__Field::__field4),
                                "install_method" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"last_modified" => _serde::__private::Ok(__Field::__field0),
                                b"revision" => _serde::__private::Ok(__Field::__field1),
                                b"url" => _serde::__private::Ok(__Field::__field2),
                                b"tag" => _serde::__private::Ok(__Field::__field3),
                                b"locked" => _serde::__private::Ok(__Field::__field4),
                                b"install_method" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ParserState>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ParserState;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ParserState",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                DateTime<Utc>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ParserState with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ParserState with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ParserState with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct ParserState with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct ParserState with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                ParserInstallMethod,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct ParserState with 6 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ParserState {
                                last_modified: __field0,
                                revision: __field1,
                                url: __field2,
                                tag: __field3,
                                locked: __field4,
                                install_method: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<DateTime<Utc>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<
                                ParserInstallMethod,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "last_modified",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                DateTime<Utc>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "revision",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("tag"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("locked"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "install_method",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                ParserInstallMethod,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("last_modified")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("revision")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("url")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("tag")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("locked")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("install_method")?
                                }
                            };
                            _serde::__private::Ok(ParserState {
                                last_modified: __field0,
                                revision: __field1,
                                url: __field2,
                                tag: __field3,
                                locked: __field4,
                                install_method: __field5,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "last_modified",
                        "revision",
                        "url",
                        "tag",
                        "locked",
                        "install_method",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ParserState",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ParserState>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Default for ParserState {
            fn default() -> Self {
                Self {
                    last_modified: Utc::now(),
                    revision: String::new(),
                    url: String::new(),
                    tag: String::new(),
                    locked: false,
                    install_method: ParserInstallMethod::Compile,
                }
            }
        }
        pub struct RestorePoint {
            pub date: DateTime<Utc>,
            pub location: PathBuf,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RestorePoint {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "RestorePoint",
                    "date",
                    &self.date,
                    "location",
                    &&self.location,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for RestorePoint {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "RestorePoint",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "date",
                        &self.date,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "location",
                        &self.location,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for RestorePoint {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "date" => _serde::__private::Ok(__Field::__field0),
                                "location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"date" => _serde::__private::Ok(__Field::__field0),
                                b"location" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<RestorePoint>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = RestorePoint;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct RestorePoint",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                DateTime<Utc>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct RestorePoint with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                PathBuf,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct RestorePoint with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(RestorePoint {
                                date: __field0,
                                location: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<DateTime<Utc>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<PathBuf> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("date"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                DateTime<Utc>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "location",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<PathBuf>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("date")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("location")?
                                }
                            };
                            _serde::__private::Ok(RestorePoint {
                                date: __field0,
                                location: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["date", "location"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "RestorePoint",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<RestorePoint>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct State {
            last_updated: DateTime<Utc>,
            pub restore_points: VecDeque<RestorePoint>,
            pub parsers: BTreeMap<String, ParserState>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for State {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "State",
                    "last_updated",
                    &self.last_updated,
                    "restore_points",
                    &self.restore_points,
                    "parsers",
                    &&self.parsers,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for State {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "State",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "last_updated",
                        &self.last_updated,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "restore_points",
                        &self.restore_points,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "parsers",
                        &self.parsers,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for State {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "last_updated" => _serde::__private::Ok(__Field::__field0),
                                "restore_points" => _serde::__private::Ok(__Field::__field1),
                                "parsers" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"last_updated" => _serde::__private::Ok(__Field::__field0),
                                b"restore_points" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                b"parsers" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<State>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = State;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct State",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                DateTime<Utc>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct State with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                VecDeque<RestorePoint>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct State with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                BTreeMap<String, ParserState>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct State with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(State {
                                last_updated: __field0,
                                restore_points: __field1,
                                parsers: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<DateTime<Utc>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                VecDeque<RestorePoint>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                BTreeMap<String, ParserState>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "last_updated",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                DateTime<Utc>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "restore_points",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                VecDeque<RestorePoint>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "parsers",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                BTreeMap<String, ParserState>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("last_updated")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("restore_points")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("parsers")?
                                }
                            };
                            _serde::__private::Ok(State {
                                last_updated: __field0,
                                restore_points: __field1,
                                parsers: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "last_updated",
                        "restore_points",
                        "parsers",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "State",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<State>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl State {
            pub fn new() -> anyhow::Result<Self> {
                let state_file = PATHS.ts_parsers.join("state.json");
                if state_file.is_file() {
                    let state_str = std::fs::read_to_string(&state_file)?;
                    let state: State = serde_json::from_str(&state_str)?;
                    return Ok(state);
                }
                Ok(Self {
                    last_updated: Utc::now(),
                    restore_points: VecDeque::new(),
                    parsers: BTreeMap::new(),
                })
            }
            pub fn check_all_installed(
                &self,
                parsers: &Vec<String>,
            ) -> (Vec<String>, Vec<String>) {
                let mut not_installed = Vec::new();
                let mut is_installed = Vec::new();
                for parser in parsers {
                    match self.parsers.contains_key(parser) {
                        true => is_installed.push(parser.clone()),
                        false => not_installed.push(parser.clone()),
                    }
                }
                (is_installed, not_installed)
            }
            pub fn add_parser(
                &mut self,
                name: &str,
                tag: &str,
                install_method: ParserInstallMethod,
                parser_info: &ParserInfo,
            ) {
                let parser_state = ParserState {
                    last_modified: Utc::now(),
                    revision: parser_info.revision.clone(),
                    tag: tag.to_owned(),
                    url: parser_info.url.clone(),
                    install_method,
                    ..ParserState::default()
                };
                self.parsers.insert(name.to_owned(), parser_state);
            }
            pub fn is_up_to_date(&self, name: &str, tag: &str) -> bool {
                let parser = self.parsers.get(name).unwrap();
                parser.tag == tag
            }
            pub fn update_parser(
                &mut self,
                name: &str,
                tag: &str,
                install_method: ParserInstallMethod,
                parser_info: &ParserInfo,
            ) {
                let parser = self.parsers.get_mut(name).unwrap();
                parser.last_modified = Utc::now();
                parser.revision = parser_info.revision.clone();
                parser.tag = tag.to_owned();
                parser.url = parser_info.url.clone();
                parser.install_method = install_method;
            }
            pub fn remove_parser(&mut self, name: &str) {
                self.parsers.remove(name);
            }
            pub fn is_locked(&self, name: &str) -> bool {
                self.parsers.get(name).unwrap().locked
            }
            pub fn lock_parser(&mut self, name: &str) {
                self.parsers.get_mut(name).unwrap().locked = true;
            }
            pub fn unlock_parser(&mut self, name: &str) {
                self.parsers.get_mut(name).unwrap().locked = false;
            }
            pub fn append_restore_point(&mut self, restore_point: RestorePoint) {
                self.restore_points.push_front(restore_point);
            }
            pub fn create_backup(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
                let state_str = serde_json::to_string_pretty(&self.parsers)?;
                std::fs::write(path, state_str)?;
                Ok(())
            }
            pub fn restore_backup(
                &mut self,
                path: impl AsRef<Path>,
            ) -> anyhow::Result<()> {
                let state_str = std::fs::read_to_string(path)?;
                self.parsers = serde_json::from_str(&state_str)?;
                Ok(())
            }
            pub fn get_restore_point(&self, id: usize) -> anyhow::Result<&RestorePoint> {
                self.restore_points
                    .get(id)
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!("Invalid restore point \'{0}\'", id),
                        );
                        error
                    }))
            }
            pub fn check_restore_exists(&self, id: usize) -> anyhow::Result<bool> {
                if self.restore_points.is_empty() {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("No restore points found"),
                        );
                        error
                    });
                }
                match self.restore_points.get(id) {
                    Some(_) => Ok(true),
                    None => Ok(false),
                }
            }
            pub fn delete_restore_point(&mut self, id: usize) {
                self.restore_points.remove(id);
            }
            pub fn delete_all_restore_points(&mut self) {
                self.restore_points.clear();
            }
            pub fn list_restore_points(&self) -> Vec<(String, String)> {
                let mut restore_points = ::alloc::vec::Vec::new();
                for restore_point in self.restore_points.iter() {
                    let date = restore_point.date.format("%F %H:%M:%S").to_string();
                    let file_name = restore_point
                        .location
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    restore_points.push((date, file_name));
                }
                restore_points
            }
            pub fn commit(&mut self) -> anyhow::Result<()> {
                self.last_updated = Utc::now();
                let state_file = PATHS.ts_parsers.join("state.json");
                std::fs::write(state_file, serde_json::to_string_pretty(&self)?)?;
                Ok(())
            }
        }
    }
}
mod ops {
    pub mod backups_ops {
        use chrono::Utc;
        use crate::{
            c_println, data::state::{RestorePoint, State},
            utils::{archives, fs as ufs, PATHS},
        };
        pub fn create_backup(state: &mut State, tag: &str) -> anyhow::Result<()> {
            let timestamp = Utc::now();
            let parsers_dir = PATHS.ts_parsers.join("parsers");
            let backup_dir = PATHS.ts_parsers.join("backups");
            let state_bak = PATHS.ts_parsers.join("state-parsers.json");
            let archive_path = backup_dir
                .join({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "backup-[{0}]-[{1}].tar.bz2",
                            timestamp.format("%F_%H-%M-%S"),
                            tag,
                        ),
                    );
                    res
                });
            if !backup_dir.exists() {
                std::fs::create_dir_all(&backup_dir)?;
            }
            if !parsers_dir.exists() || parsers_dir.read_dir()?.next().is_none() {
                {
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0}\n",
                                ::ansi_term::Colour::RGB(2, 149, 235)
                                    .paint(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!("INFO: No parsers to backup"),
                                            );
                                            res
                                        },
                                    ),
                            ),
                        );
                    }
                };
                return Ok(());
            }
            state.create_backup(&state_bak)?;
            archives::create_tar_bz2(&archive_path, &[&parsers_dir, &state_bak])?;
            state
                .append_restore_point(RestorePoint {
                    date: timestamp,
                    location: archive_path,
                });
            std::fs::remove_file(&state_bak)?;
            Ok(())
        }
        pub fn restore_backup(state: &mut State, id: usize) -> anyhow::Result<()> {
            let restore_point = state.get_restore_point(id)?;
            let tmp_dir = tempfile::tempdir()?;
            archives::extract_tar_bz2(&restore_point.location, tmp_dir.path())?;
            ufs::remove_all(PATHS.ts_parsers.join("parsers"))?;
            std::fs::create_dir_all(PATHS.ts_parsers.join("parsers"))?;
            ufs::copy_all(
                tmp_dir.path().join("parsers"),
                PATHS.ts_parsers.join("parsers"),
            )?;
            std::fs::remove_file(&restore_point.location)?;
            state.restore_backup(tmp_dir.path().join("state-parsers.json"))?;
            state.delete_restore_point(id);
            Ok(())
        }
        pub fn delete_backup(
            state: &mut State,
            ids: Vec<usize>,
            all: bool,
        ) -> anyhow::Result<()> {
            if all {
                state.delete_all_restore_points();
            } else {
                for id in ids {
                    state.delete_restore_point(id);
                }
            }
            Ok(())
        }
    }
    pub mod parser_ops {
        use std::path::{Path, PathBuf};
        use crate::{
            c_println, compiler::{Compiler, ZigTargets},
            data::parsers::ParserInfo, utils::{archives, command, http},
        };
        #[cfg(unix)]
        const PNPM: &str = "pnpm";
        pub fn check_compile_deps(compiler: &dyn Compiler) -> anyhow::Result<()> {
            command::check_exists(compiler.get_name())?;
            command::check_exists(PNPM)?;
            command::check_exists("tree-sitter")?;
            Ok(())
        }
        pub fn uninstall(lang: &str, destination: &Path) -> anyhow::Result<()> {
            let lang_so = {
                let res = ::alloc::fmt::format(format_args!("{0}.so", lang));
                res
            };
            let lang_so = PathBuf::from(lang_so);
            let lang_so = destination.join(lang_so);
            if lang_so.exists() {
                std::fs::remove_file(lang_so)?;
            } else {
                return ::anyhow::__private::Err({
                    let error = ::anyhow::__private::format_err(
                        format_args!("Parser {0} is not installed", lang),
                    );
                    error
                });
            }
            Ok(())
        }
        pub async fn compile(
            lang: &str,
            parser_info: &ParserInfo,
            compiler: &dyn Compiler,
            target: &Option<ZigTargets>,
            destination: &Path,
        ) -> anyhow::Result<()> {
            let tmp_dir = tempfile::tempdir()?;
            let download_location = tmp_dir
                .path()
                .join({
                    let res = ::alloc::fmt::format(format_args!("{0}.tar.gz", lang));
                    res
                });
            let download_url = download_url(parser_info);
            let extract_dir = tmp_dir.path().join(lang);
            let mut cwd = PathBuf::new();
            http::download_file(&download_url, &download_location).await?;
            archives::extract_tar_gz(&download_location, &extract_dir)?;
            for entry in std::fs::read_dir(&extract_dir)? {
                let entry = entry?;
                cwd = entry.path();
            }
            if let Some(location) = &parser_info.location {
                cwd = cwd.join(location);
            }
            if parser_info.generate_from_grammar {
                if let Err(e) = generate_from_grammar(&cwd).await {
                    {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "{0}\n",
                                    ::ansi_term::Colour::RGB(245, 181, 61)
                                        .paint(
                                            &{
                                                let res = ::alloc::fmt::format(
                                                    format_args!(
                                                        "=> WARNGING: tree-sitter generate failed: {0}",
                                                        e,
                                                    ),
                                                );
                                                res
                                            },
                                        ),
                                ),
                            );
                        }
                    };
                }
            }
            let files = parser_info.files.iter().map(|f| f.as_str()).collect::<Vec<_>>();
            compiler.compile(&files, &cwd, target).await?;
            let destination = PathBuf::from(
                shellexpand::full(destination.to_str().unwrap())?.to_string(),
            );
            std::fs::create_dir_all(&destination)?;
            std::fs::copy(
                cwd.join("out.so"),
                destination
                    .join({
                        let res = ::alloc::fmt::format(format_args!("{0}.so", lang));
                        res
                    }),
            )?;
            Ok(())
        }
        fn download_url(parser_info: &ParserInfo) -> String {
            let url = parser_info.url.trim_end_matches(".git");
            let repo_name = url.split('/').last().unwrap();
            match parser_info.url.contains("gitlab") {
                true => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{2}/-/archive/{0}/{3}-{1}.tar.gz",
                            parser_info.revision,
                            parser_info.revision,
                            url,
                            repo_name,
                        ),
                    );
                    res
                }
                false => {
                    let res = ::alloc::fmt::format(
                        format_args!("{1}/archive/{0}.tar.gz", parser_info.revision, url),
                    );
                    res
                }
            }
        }
        async fn generate_from_grammar(cwd: &Path) -> anyhow::Result<()> {
            command::run(PNPM, &["install"], Some(&cwd)).await?;
            command::run("tree-sitter", &["generate"], Some(&cwd)).await?;
            Ok(())
        }
    }
}
mod subcommands {
    mod backups {
        use tabled::{builder::Builder as TableBuilder, settings::Style as TableStyle};
        use crate::{c_println, data::state::State, ops::backups_ops};
        use super::Subcommand;
        pub struct Backups {
            #[command(subcommand)]
            commands: Commands,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Backups {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Backups",
                    "commands",
                    &&self.commands,
                )
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for Backups {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = Backups {
                    commands: {
                        <Commands as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?
                    },
                };
                ::std::result::Result::Ok(v)
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                {
                    #[allow(non_snake_case)]
                    let commands = &mut self.commands;
                    <Commands as clap::FromArgMatches>::update_from_arg_matches_mut(
                        commands,
                        __clap_arg_matches,
                    )?;
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Args for Backups {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("Backups"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Backups")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0usize] = [];
                                    members
                                }),
                        );
                    let __clap_app = <Commands as clap::Subcommand>::augment_subcommands(
                        __clap_app,
                    );
                    let __clap_app = __clap_app
                        .subcommand_required(true)
                        .arg_required_else_help(true);
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Backups")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 0usize] = [];
                                    members
                                }),
                        );
                    let __clap_app = <Commands as clap::Subcommand>::augment_subcommands(
                        __clap_app,
                    );
                    let __clap_app = __clap_app
                        .subcommand_required(true)
                        .arg_required_else_help(true)
                        .subcommand_required(false)
                        .arg_required_else_help(false);
                    __clap_app
                }
            }
        }
        enum Commands {
            /// Create a new backup
            Create,
            /// Restore from a selected restore-point
            Restore {
                /// ID of the backup to restore
                id: i32,
            },
            /// List all available backups
            List,
            /// Delete backups
            Delete {
                /// IDs of the backups to delete.
                /// Input can be a single ID, a comma-separated list of IDs or a range of IDs.
                ///
                /// eg.
                /// - '1' (delete backup with ID 1)
                /// - '1,2,3' (delete backups with IDs 1, 2 and 3)
                /// - '1..3' (delete backups with IDs 1, 2 and 3)
                /// - '1, 2..4' (delete backups with IDs 1, 2, 3 and 4)
                #[clap(conflicts_with = "all", verbatim_doc_comment)]
                ids: Option<String>,
                /// Delete all backups
                #[clap(short, long, conflicts_with = "ids")]
                all: bool,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Commands {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Commands::Create => ::core::fmt::Formatter::write_str(f, "Create"),
                    Commands::Restore { id: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Restore",
                            "id",
                            &__self_0,
                        )
                    }
                    Commands::List => ::core::fmt::Formatter::write_str(f, "List"),
                    Commands::Delete { ids: __self_0, all: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Delete",
                            "ids",
                            __self_0,
                            "all",
                            &__self_1,
                        )
                    }
                }
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for Commands {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                if let Some((__clap_name, mut __clap_arg_sub_matches)) = __clap_arg_matches
                    .remove_subcommand()
                {
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    if __clap_name == "create" && !__clap_arg_matches.contains_id("") {
                        return ::std::result::Result::Ok(Self::Create);
                    }
                    if __clap_name == "restore" && !__clap_arg_matches.contains_id("") {
                        return ::std::result::Result::Ok(Self::Restore {
                            id: __clap_arg_matches
                                .remove_one::<i32>("id")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    "The following required argument was not provided: id",
                                ))?,
                        });
                    }
                    if __clap_name == "list" && !__clap_arg_matches.contains_id("") {
                        return ::std::result::Result::Ok(Self::List);
                    }
                    if __clap_name == "delete" && !__clap_arg_matches.contains_id("") {
                        return ::std::result::Result::Ok(Self::Delete {
                            ids: __clap_arg_matches.remove_one::<String>("ids"),
                            all: __clap_arg_matches
                                .remove_one::<bool>("all")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    "The following required argument was not provided: all",
                                ))?,
                        });
                    }
                    ::std::result::Result::Err(
                        clap::Error::raw(
                            clap::error::ErrorKind::InvalidSubcommand,
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "The subcommand \'{0}\' wasn\'t recognized",
                                        __clap_name,
                                    ),
                                );
                                res
                            },
                        ),
                    )
                } else {
                    ::std::result::Result::Err(
                        clap::Error::raw(
                            clap::error::ErrorKind::MissingSubcommand,
                            "A subcommand is required but one was not provided.",
                        ),
                    )
                }
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut<'b>(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                    match self {
                        Self::Create if "create" == __clap_name => {
                            let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                                .remove_subcommand()
                                .unwrap();
                            let __clap_arg_matches = &mut __clap_arg_sub_matches;
                            {}
                        }
                        Self::Restore { id } if "restore" == __clap_name => {
                            let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                                .remove_subcommand()
                                .unwrap();
                            let __clap_arg_matches = &mut __clap_arg_sub_matches;
                            {
                                if __clap_arg_matches.contains_id("id") {
                                    *id = __clap_arg_matches
                                        .remove_one::<i32>("id")
                                        .ok_or_else(|| clap::Error::raw(
                                            clap::error::ErrorKind::MissingRequiredArgument,
                                            "The following required argument was not provided: id",
                                        ))?;
                                }
                            }
                        }
                        Self::List if "list" == __clap_name => {
                            let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                                .remove_subcommand()
                                .unwrap();
                            let __clap_arg_matches = &mut __clap_arg_sub_matches;
                            {}
                        }
                        Self::Delete { ids, all } if "delete" == __clap_name => {
                            let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                                .remove_subcommand()
                                .unwrap();
                            let __clap_arg_matches = &mut __clap_arg_sub_matches;
                            {
                                if __clap_arg_matches.contains_id("ids") {
                                    *ids = __clap_arg_matches.remove_one::<String>("ids");
                                }
                                if __clap_arg_matches.contains_id("all") {
                                    *all = __clap_arg_matches
                                        .remove_one::<bool>("all")
                                        .ok_or_else(|| clap::Error::raw(
                                            clap::error::ErrorKind::MissingRequiredArgument,
                                            "The following required argument was not provided: all",
                                        ))?;
                                }
                            }
                        }
                        s => {
                            *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                                __clap_arg_matches,
                            )?;
                        }
                    }
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Subcommand for Commands {
            fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
                let __clap_app = __clap_app;
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("create");
                        let __clap_subcommand = __clap_subcommand;
                        let __clap_subcommand = __clap_subcommand;
                        __clap_subcommand.about("Create a new backup").long_about(None)
                    });
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("restore");
                        {
                            let __clap_subcommand = __clap_subcommand
                                .group(
                                    clap::ArgGroup::new("Restore")
                                        .multiple(true)
                                        .args({
                                            let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                                            members
                                        }),
                                );
                            let __clap_subcommand = __clap_subcommand
                                .arg({
                                    #[allow(deprecated)]
                                    let arg = clap::Arg::new("id")
                                        .value_name("ID")
                                        .required(true && clap::ArgAction::Set.takes_values())
                                        .value_parser({
                                            use ::clap_builder::builder::via_prelude::*;
                                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                                i32,
                                            >::new();
                                            (&&&&&&auto).value_parser()
                                        })
                                        .action(clap::ArgAction::Set);
                                    let arg = arg
                                        .help("ID of the backup to restore")
                                        .long_help(None);
                                    let arg = arg;
                                    arg
                                });
                            __clap_subcommand
                                .about("Restore from a selected restore-point")
                                .long_about(None)
                        }
                    });
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("list");
                        let __clap_subcommand = __clap_subcommand;
                        let __clap_subcommand = __clap_subcommand;
                        __clap_subcommand
                            .about("List all available backups")
                            .long_about(None)
                    });
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("delete");
                        {
                            let __clap_subcommand = __clap_subcommand
                                .group(
                                    clap::ArgGroup::new("Delete")
                                        .multiple(true)
                                        .args({
                                            let members: [clap::Id; 2usize] = [
                                                clap::Id::from("ids"),
                                                clap::Id::from("all"),
                                            ];
                                            members
                                        }),
                                );
                            let __clap_subcommand = __clap_subcommand
                                .arg({
                                    #[allow(deprecated)]
                                    let arg = clap::Arg::new("ids")
                                        .value_name("IDS")
                                        .value_parser({
                                            use ::clap_builder::builder::via_prelude::*;
                                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                                String,
                                            >::new();
                                            (&&&&&&auto).value_parser()
                                        })
                                        .action(clap::ArgAction::Set);
                                    let arg = arg
                                        .help(
                                            "IDs of the backups to delete.\nInput can be a single ID, a comma-separated list of IDs or a range of IDs.",
                                        )
                                        .long_help(
                                            "IDs of the backups to delete.\nInput can be a single ID, a comma-separated list of IDs or a range of IDs.\n\neg.\n- '1' (delete backup with ID 1)\n- '1,2,3' (delete backups with IDs 1, 2 and 3)\n- '1..3' (delete backups with IDs 1, 2 and 3)\n- '1, 2..4' (delete backups with IDs 1, 2, 3 and 4)",
                                        )
                                        .conflicts_with("all");
                                    let arg = arg;
                                    arg
                                });
                            let __clap_subcommand = __clap_subcommand
                                .arg({
                                    #[allow(deprecated)]
                                    let arg = clap::Arg::new("all")
                                        .value_name("ALL")
                                        .required(true && clap::ArgAction::SetTrue.takes_values())
                                        .value_parser({
                                            use ::clap_builder::builder::via_prelude::*;
                                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                                bool,
                                            >::new();
                                            (&&&&&&auto).value_parser()
                                        })
                                        .action(clap::ArgAction::SetTrue);
                                    let arg = arg
                                        .help("Delete all backups")
                                        .long_help(None)
                                        .short('a')
                                        .long("all")
                                        .conflicts_with("ids");
                                    let arg = arg;
                                    arg
                                });
                            __clap_subcommand.about("Delete backups").long_about(None)
                        }
                    });
                __clap_app
            }
            fn augment_subcommands_for_update<'b>(
                __clap_app: clap::Command,
            ) -> clap::Command {
                let __clap_app = __clap_app;
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("create");
                        let __clap_subcommand = __clap_subcommand;
                        let __clap_subcommand = __clap_subcommand;
                        __clap_subcommand.about("Create a new backup").long_about(None)
                    });
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("restore");
                        {
                            let __clap_subcommand = __clap_subcommand
                                .group(
                                    clap::ArgGroup::new("Restore")
                                        .multiple(true)
                                        .args({
                                            let members: [clap::Id; 1usize] = [clap::Id::from("id")];
                                            members
                                        }),
                                );
                            let __clap_subcommand = __clap_subcommand
                                .arg({
                                    #[allow(deprecated)]
                                    let arg = clap::Arg::new("id")
                                        .value_name("ID")
                                        .required(true && clap::ArgAction::Set.takes_values())
                                        .value_parser({
                                            use ::clap_builder::builder::via_prelude::*;
                                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                                i32,
                                            >::new();
                                            (&&&&&&auto).value_parser()
                                        })
                                        .action(clap::ArgAction::Set);
                                    let arg = arg
                                        .help("ID of the backup to restore")
                                        .long_help(None);
                                    let arg = arg.required(false);
                                    arg
                                });
                            __clap_subcommand
                                .about("Restore from a selected restore-point")
                                .long_about(None)
                        }
                    });
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("list");
                        let __clap_subcommand = __clap_subcommand;
                        let __clap_subcommand = __clap_subcommand;
                        __clap_subcommand
                            .about("List all available backups")
                            .long_about(None)
                    });
                let __clap_app = __clap_app
                    .subcommand({
                        let __clap_subcommand = clap::Command::new("delete");
                        {
                            let __clap_subcommand = __clap_subcommand
                                .group(
                                    clap::ArgGroup::new("Delete")
                                        .multiple(true)
                                        .args({
                                            let members: [clap::Id; 2usize] = [
                                                clap::Id::from("ids"),
                                                clap::Id::from("all"),
                                            ];
                                            members
                                        }),
                                );
                            let __clap_subcommand = __clap_subcommand
                                .arg({
                                    #[allow(deprecated)]
                                    let arg = clap::Arg::new("ids")
                                        .value_name("IDS")
                                        .value_parser({
                                            use ::clap_builder::builder::via_prelude::*;
                                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                                String,
                                            >::new();
                                            (&&&&&&auto).value_parser()
                                        })
                                        .action(clap::ArgAction::Set);
                                    let arg = arg
                                        .help(
                                            "IDs of the backups to delete.\nInput can be a single ID, a comma-separated list of IDs or a range of IDs.",
                                        )
                                        .long_help(
                                            "IDs of the backups to delete.\nInput can be a single ID, a comma-separated list of IDs or a range of IDs.\n\neg.\n- '1' (delete backup with ID 1)\n- '1,2,3' (delete backups with IDs 1, 2 and 3)\n- '1..3' (delete backups with IDs 1, 2 and 3)\n- '1, 2..4' (delete backups with IDs 1, 2, 3 and 4)",
                                        )
                                        .conflicts_with("all");
                                    let arg = arg.required(false);
                                    arg
                                });
                            let __clap_subcommand = __clap_subcommand
                                .arg({
                                    #[allow(deprecated)]
                                    let arg = clap::Arg::new("all")
                                        .value_name("ALL")
                                        .required(true && clap::ArgAction::SetTrue.takes_values())
                                        .value_parser({
                                            use ::clap_builder::builder::via_prelude::*;
                                            let auto = ::clap_builder::builder::_AutoValueParser::<
                                                bool,
                                            >::new();
                                            (&&&&&&auto).value_parser()
                                        })
                                        .action(clap::ArgAction::SetTrue);
                                    let arg = arg
                                        .help("Delete all backups")
                                        .long_help(None)
                                        .short('a')
                                        .long("all")
                                        .conflicts_with("ids");
                                    let arg = arg.required(false);
                                    arg
                                });
                            __clap_subcommand.about("Delete backups").long_about(None)
                        }
                    });
                __clap_app
            }
            fn has_subcommand(__clap_name: &str) -> bool {
                if "create" == __clap_name {
                    return true;
                }
                if "restore" == __clap_name {
                    return true;
                }
                if "list" == __clap_name {
                    return true;
                }
                if "delete" == __clap_name {
                    return true;
                }
                false
            }
        }
        impl Subcommand for Backups {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn run<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        let mut state = State::new()?;
                        match &__self.commands {
                            Commands::Create => {
                                {
                                    {
                                        ::std::io::_print(
                                            format_args!(
                                                "{0}\n",
                                                ::ansi_term::Colour::RGB(2, 149, 235)
                                                    .paint(
                                                        &{
                                                            let res = ::alloc::fmt::format(
                                                                format_args!("Creating backup..."),
                                                            );
                                                            res
                                                        },
                                                    ),
                                            ),
                                        );
                                    }
                                };
                                backups_ops::create_backup(&mut state, "manual")?;
                                state.commit()?;
                                {
                                    {
                                        ::std::io::_print(
                                            format_args!(
                                                "{0}\n",
                                                ::ansi_term::Colour::RGB(57, 219, 57)
                                                    .paint(
                                                        &{
                                                            let res = ::alloc::fmt::format(
                                                                format_args!("Backup created!"),
                                                            );
                                                            res
                                                        },
                                                    ),
                                            ),
                                        );
                                    }
                                };
                                Ok(())
                            }
                            Commands::List => {
                                let backups = state.list_restore_points();
                                if backups.is_empty() {
                                    {
                                        {
                                            ::std::io::_print(
                                                format_args!(
                                                    "{0}\n",
                                                    ::ansi_term::Colour::RGB(245, 181, 61)
                                                        .paint(
                                                            &{
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!("No backups found"),
                                                                );
                                                                res
                                                            },
                                                        ),
                                                ),
                                            );
                                        }
                                    };
                                    return Ok(());
                                }
                                let mut builder = TableBuilder::default();
                                builder.push_record(["ID", "Date", "File"]);
                                for (i, (date, file)) in backups.iter().enumerate() {
                                    builder
                                        .push_record([
                                            (i + 1).to_string(),
                                            date.to_owned(),
                                            file.to_owned(),
                                        ]);
                                }
                                let mut table = builder.build();
                                table.with(TableStyle::rounded());
                                {
                                    ::std::io::_print(format_args!("{0}\n", table));
                                };
                                Ok(())
                            }
                            Commands::Restore { id } => {
                                let mut state = State::new()?;
                                let id = id - 1;
                                if id < 0 {
                                    return ::anyhow::__private::Err({
                                        let error = ::anyhow::__private::format_err(
                                            format_args!("ID must be a positive integer"),
                                        );
                                        error
                                    });
                                }
                                if !state.check_restore_exists(id as usize)? {
                                    return ::anyhow::__private::Err(
                                        ::anyhow::Error::msg({
                                            let res = ::alloc::fmt::format(
                                                format_args!("Backup with ID {0} does not exist", id + 1),
                                            );
                                            res
                                        }),
                                    );
                                }
                                {
                                    {
                                        ::std::io::_print(
                                            format_args!(
                                                "{0}\n",
                                                ::ansi_term::Colour::RGB(2, 149, 235)
                                                    .paint(
                                                        &{
                                                            let res = ::alloc::fmt::format(
                                                                format_args!("Restoring backup..."),
                                                            );
                                                            res
                                                        },
                                                    ),
                                            ),
                                        );
                                    }
                                };
                                backups_ops::restore_backup(&mut state, id as usize)?;
                                state.commit()?;
                                {
                                    {
                                        ::std::io::_print(
                                            format_args!(
                                                "{0}\n",
                                                ::ansi_term::Colour::RGB(57, 219, 57)
                                                    .paint(
                                                        &{
                                                            let res = ::alloc::fmt::format(
                                                                format_args!("Backup restored!"),
                                                            );
                                                            res
                                                        },
                                                    ),
                                            ),
                                        );
                                    }
                                };
                                Ok(())
                            }
                            Commands::Delete { ids, all } => {
                                if *all {
                                    {
                                        {
                                            ::std::io::_print(
                                                format_args!(
                                                    "{0}\n",
                                                    ::ansi_term::Colour::RGB(2, 149, 235)
                                                        .paint(
                                                            &{
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!("Deleting all backups..."),
                                                                );
                                                                res
                                                            },
                                                        ),
                                                ),
                                            );
                                        }
                                    };
                                    backups_ops::delete_backup(
                                        &mut state,
                                        ::alloc::vec::Vec::new(),
                                        true,
                                    )?;
                                    state.commit()?;
                                    {
                                        {
                                            ::std::io::_print(
                                                format_args!(
                                                    "{0}\n",
                                                    ::ansi_term::Colour::RGB(57, 219, 57)
                                                        .paint(
                                                            &{
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!("All backups deleted!"),
                                                                );
                                                                res
                                                            },
                                                        ),
                                                ),
                                            );
                                        }
                                    };
                                    return Ok(());
                                }
                                match ids {
                                    Some(_query) => {}
                                    None => {
                                        return ::anyhow::__private::Err({
                                            let error = ::anyhow::__private::format_err(
                                                format_args!(
                                                    "You must provide an ID or range of IDs to delete",
                                                ),
                                            );
                                            error
                                        });
                                    }
                                }
                                Ok(())
                            }
                        }
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    mod compile {
        use std::path::PathBuf;
        use crate::{
            c_println, compiler::{select_compiler, CompilerOption, ZigTargets},
            data::{changelog::ChangeLog, parsers::Parsers},
            ops::parser_ops,
        };
        use super::Subcommand;
        pub struct Compile {
            /// Compiler to use
            #[clap(short, long, default_value_t, value_enum)]
            compiler: CompilerOption,
            /// Zig compile target
            /// Only used when compiler is zig.
            /// (defaults to host architecture)
            #[clap(short, long, value_enum, verbatim_doc_comment)]
            target: Option<ZigTargets>,
            /// Compile all parsers
            #[clap(short, long, default_value = "false")]
            all: bool,
            /// Output directory to compile parsers to
            #[clap(short, long)]
            destination: PathBuf,
            /// 'nvim-treesitter-parsers' tags to use.
            /// Will only use tags present in the changelog.
            /// (defaults to latest tag)
            ///
            /// See https://github.com/KevinSilvester/nvim-treesitter-parerers
            #[clap(long, verbatim_doc_comment)]
            tag: Option<String>,
            /// Compile parsers in 'wanted-parsers.txt'
            #[clap(
                short,
                long,
                default_value = "false",
                conflicts_with_all = ["all",
                "parsers"]
            )]
            wanted: bool,
            /// Parsers to compile (cannot be used with --all or --wanted)
            #[clap(conflicts_with = "all")]
            parsers: Vec<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Compile {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "compiler",
                    "target",
                    "all",
                    "destination",
                    "tag",
                    "wanted",
                    "parsers",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.compiler,
                    &self.target,
                    &self.all,
                    &self.destination,
                    &self.tag,
                    &self.wanted,
                    &&self.parsers,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Compile",
                    names,
                    values,
                )
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for Compile {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = Compile {
                    compiler: __clap_arg_matches
                        .remove_one::<CompilerOption>("compiler")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: compiler",
                        ))?,
                    target: __clap_arg_matches.remove_one::<ZigTargets>("target"),
                    all: __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?,
                    destination: __clap_arg_matches
                        .remove_one::<PathBuf>("destination")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: destination",
                        ))?,
                    tag: __clap_arg_matches.remove_one::<String>("tag"),
                    wanted: __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?,
                    parsers: __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new),
                };
                ::std::result::Result::Ok(v)
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if __clap_arg_matches.contains_id("compiler") {
                    #[allow(non_snake_case)]
                    let compiler = &mut self.compiler;
                    *compiler = __clap_arg_matches
                        .remove_one::<CompilerOption>("compiler")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: compiler",
                        ))?;
                }
                if __clap_arg_matches.contains_id("target") {
                    #[allow(non_snake_case)]
                    let target = &mut self.target;
                    *target = __clap_arg_matches.remove_one::<ZigTargets>("target");
                }
                if __clap_arg_matches.contains_id("all") {
                    #[allow(non_snake_case)]
                    let all = &mut self.all;
                    *all = __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?;
                }
                if __clap_arg_matches.contains_id("destination") {
                    #[allow(non_snake_case)]
                    let destination = &mut self.destination;
                    *destination = __clap_arg_matches
                        .remove_one::<PathBuf>("destination")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: destination",
                        ))?;
                }
                if __clap_arg_matches.contains_id("tag") {
                    #[allow(non_snake_case)]
                    let tag = &mut self.tag;
                    *tag = __clap_arg_matches.remove_one::<String>("tag");
                }
                if __clap_arg_matches.contains_id("wanted") {
                    #[allow(non_snake_case)]
                    let wanted = &mut self.wanted;
                    *wanted = __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?;
                }
                if __clap_arg_matches.contains_id("parsers") {
                    #[allow(non_snake_case)]
                    let parsers = &mut self.parsers;
                    *parsers = __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new);
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Args for Compile {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("Compile"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Compile")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 7usize] = [
                                        clap::Id::from("compiler"),
                                        clap::Id::from("target"),
                                        clap::Id::from("all"),
                                        clap::Id::from("destination"),
                                        clap::Id::from("tag"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("compiler")
                                .value_name("COMPILER")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        CompilerOption,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Compiler to use")
                                .long_help(None)
                                .short('c')
                                .long("compiler")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: CompilerOption = <CompilerOption as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("target")
                                .value_name("TARGET")
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        ZigTargets,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help(
                                    "Zig compile target\nOnly used when compiler is zig.\n(defaults to host architecture)",
                                )
                                .long_help(None)
                                .short('t')
                                .long("target");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Compile all parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("destination")
                                .value_name("DESTINATION")
                                .required(true && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        PathBuf,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Output directory to compile parsers to")
                                .long_help(None)
                                .short('d')
                                .long("destination");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("tag")
                                .value_name("TAG")
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help(
                                    "'nvim-treesitter-parsers' tags to use.\nWill only use tags present in the changelog.\n(defaults to latest tag)",
                                )
                                .long_help(
                                    "'nvim-treesitter-parsers' tags to use.\nWill only use tags present in the changelog.\n(defaults to latest tag)\n\nSee https://github.com/KevinSilvester/nvim-treesitter-parerers",
                                )
                                .long("tag");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Compile parsers in 'wanted-parsers.txt'")
                                .long_help(None)
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Parsers to compile (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg;
                            arg
                        });
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Compile")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 7usize] = [
                                        clap::Id::from("compiler"),
                                        clap::Id::from("target"),
                                        clap::Id::from("all"),
                                        clap::Id::from("destination"),
                                        clap::Id::from("tag"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("compiler")
                                .value_name("COMPILER")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        CompilerOption,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Compiler to use")
                                .long_help(None)
                                .short('c')
                                .long("compiler")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: CompilerOption = <CompilerOption as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("target")
                                .value_name("TARGET")
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        ZigTargets,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help(
                                    "Zig compile target\nOnly used when compiler is zig.\n(defaults to host architecture)",
                                )
                                .long_help(None)
                                .short('t')
                                .long("target");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Compile all parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("destination")
                                .value_name("DESTINATION")
                                .required(true && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        PathBuf,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Output directory to compile parsers to")
                                .long_help(None)
                                .short('d')
                                .long("destination");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("tag")
                                .value_name("TAG")
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help(
                                    "'nvim-treesitter-parsers' tags to use.\nWill only use tags present in the changelog.\n(defaults to latest tag)",
                                )
                                .long_help(
                                    "'nvim-treesitter-parsers' tags to use.\nWill only use tags present in the changelog.\n(defaults to latest tag)\n\nSee https://github.com/KevinSilvester/nvim-treesitter-parerers",
                                )
                                .long("tag");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Compile parsers in 'wanted-parsers.txt'")
                                .long_help(None)
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Parsers to compile (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_app
                }
            }
        }
        impl Compile {
            fn select_langs(&self, parsers: &Parsers) -> anyhow::Result<Vec<String>> {
                if self.all {
                    return Ok(parsers.langs.clone());
                }
                let langs = match self.wanted {
                    true => {
                        if parsers.wanted.is_none() {
                            {
                                let red = ::anstyle::RgbColor(235, 66, 66).on_default();
                                {
                                    ::std::io::_eprint(
                                        format_args!(
                                            "{1}{0}{1:#}\n",
                                            {
                                                let res = ::alloc::fmt::format(
                                                    format_args!("No wanted parsers found"),
                                                );
                                                res
                                            },
                                            red,
                                        ),
                                    );
                                }
                            };
                            return Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("No wanted parsers found"),
                                    );
                                    error
                                }),
                            );
                        }
                        parsers.wanted.clone().unwrap()
                    }
                    false => self.parsers.clone(),
                };
                parsers.validate_parsers(&langs)?;
                if langs.is_empty() {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("No parsers found"),
                            );
                            error
                        }),
                    );
                }
                Ok(langs)
            }
        }
        impl Subcommand for Compile {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn run<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        let compiler = &*select_compiler(&__self.compiler);
                        let mut parsers = Parsers::new()?;
                        let mut changelog = ChangeLog::new();
                        changelog.fetch_changelog().await?;
                        changelog.check_entry(&__self.tag)?;
                        parsers.fetch_list(&__self.tag).await?;
                        let langs = &__self.select_langs(&parsers)?;
                        parser_ops::check_compile_deps(compiler)?;
                        for (idx, lang) in langs.iter().enumerate() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(2, 149, 235)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}/{1}. Compiling parser {2}",
                                                                (idx + 1),
                                                                langs.len(),
                                                                lang,
                                                            ),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                            let parser = parsers.get_parser(lang).unwrap();
                            parser_ops::compile(
                                    lang,
                                    parser,
                                    compiler,
                                    &__self.target,
                                    &__self.destination,
                                )
                                .await?;
                        }
                        Ok(())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    mod install {
        use crate::{
            c_println, compiler::{select_compiler, CompilerOption},
            data::{
                changelog::ChangeLog, parsers::Parsers,
                state::{ParserInstallMethod, State},
            },
            ops::{backups_ops, parser_ops},
            utils::{fs as ufs, PATHS},
        };
        use super::Subcommand;
        pub struct Install {
            /// Installation method
            #[clap(short, long, default_value_t, value_enum)]
            method: ParserInstallMethod,
            /// Compiler to use
            #[clap(short, long, default_value_t, value_enum)]
            compiler: CompilerOption,
            /// Compile all parsers
            #[clap(short, long, default_value = "false")]
            all: bool,
            /// 'nvim-treesitter-parsers' tags to use.
            /// Will only use tags present in the changelog.
            /// (defaults to latest tag)
            ///
            /// See https://github.com/KevinSilvester/nvim-treesitter-parerers
            #[clap(long, verbatim_doc_comment)]
            tag: Option<String>,
            /// Compile parsers in 'wanted-parsers.txt'
            #[clap(
                short,
                long,
                default_value = "false",
                conflicts_with_all = ["all",
                "parsers"]
            )]
            wanted: bool,
            /// Install specific parsers (cannot be used with --all or --wanted)
            #[clap(conflicts_with = "all")]
            parsers: Vec<String>,
            /// Force install, even if parsers are already installed
            #[clap(long, default_value = "false")]
            force: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Install {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "method",
                    "compiler",
                    "all",
                    "tag",
                    "wanted",
                    "parsers",
                    "force",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.method,
                    &self.compiler,
                    &self.all,
                    &self.tag,
                    &self.wanted,
                    &self.parsers,
                    &&self.force,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Install",
                    names,
                    values,
                )
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for Install {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = Install {
                    method: __clap_arg_matches
                        .remove_one::<ParserInstallMethod>("method")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: method",
                        ))?,
                    compiler: __clap_arg_matches
                        .remove_one::<CompilerOption>("compiler")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: compiler",
                        ))?,
                    all: __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?,
                    tag: __clap_arg_matches.remove_one::<String>("tag"),
                    wanted: __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?,
                    parsers: __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new),
                    force: __clap_arg_matches
                        .remove_one::<bool>("force")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: force",
                        ))?,
                };
                ::std::result::Result::Ok(v)
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if __clap_arg_matches.contains_id("method") {
                    #[allow(non_snake_case)]
                    let method = &mut self.method;
                    *method = __clap_arg_matches
                        .remove_one::<ParserInstallMethod>("method")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: method",
                        ))?;
                }
                if __clap_arg_matches.contains_id("compiler") {
                    #[allow(non_snake_case)]
                    let compiler = &mut self.compiler;
                    *compiler = __clap_arg_matches
                        .remove_one::<CompilerOption>("compiler")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: compiler",
                        ))?;
                }
                if __clap_arg_matches.contains_id("all") {
                    #[allow(non_snake_case)]
                    let all = &mut self.all;
                    *all = __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?;
                }
                if __clap_arg_matches.contains_id("tag") {
                    #[allow(non_snake_case)]
                    let tag = &mut self.tag;
                    *tag = __clap_arg_matches.remove_one::<String>("tag");
                }
                if __clap_arg_matches.contains_id("wanted") {
                    #[allow(non_snake_case)]
                    let wanted = &mut self.wanted;
                    *wanted = __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?;
                }
                if __clap_arg_matches.contains_id("parsers") {
                    #[allow(non_snake_case)]
                    let parsers = &mut self.parsers;
                    *parsers = __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new);
                }
                if __clap_arg_matches.contains_id("force") {
                    #[allow(non_snake_case)]
                    let force = &mut self.force;
                    *force = __clap_arg_matches
                        .remove_one::<bool>("force")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: force",
                        ))?;
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Args for Install {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("Install"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Install")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 7usize] = [
                                        clap::Id::from("method"),
                                        clap::Id::from("compiler"),
                                        clap::Id::from("all"),
                                        clap::Id::from("tag"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                        clap::Id::from("force"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("method")
                                .value_name("METHOD")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        ParserInstallMethod,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Installation method")
                                .long_help(None)
                                .short('m')
                                .long("method")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: ParserInstallMethod = <ParserInstallMethod as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("compiler")
                                .value_name("COMPILER")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        CompilerOption,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Compiler to use")
                                .long_help(None)
                                .short('c')
                                .long("compiler")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: CompilerOption = <CompilerOption as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Compile all parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("tag")
                                .value_name("TAG")
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help(
                                    "'nvim-treesitter-parsers' tags to use.\nWill only use tags present in the changelog.\n(defaults to latest tag)",
                                )
                                .long_help(
                                    "'nvim-treesitter-parsers' tags to use.\nWill only use tags present in the changelog.\n(defaults to latest tag)\n\nSee https://github.com/KevinSilvester/nvim-treesitter-parerers",
                                )
                                .long("tag");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Compile parsers in 'wanted-parsers.txt'")
                                .long_help(None)
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Install specific parsers (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("force")
                                .value_name("FORCE")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Force install, even if parsers are already installed",
                                )
                                .long_help(None)
                                .long("force")
                                .default_value("false");
                            let arg = arg;
                            arg
                        });
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Install")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 7usize] = [
                                        clap::Id::from("method"),
                                        clap::Id::from("compiler"),
                                        clap::Id::from("all"),
                                        clap::Id::from("tag"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                        clap::Id::from("force"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("method")
                                .value_name("METHOD")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        ParserInstallMethod,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Installation method")
                                .long_help(None)
                                .short('m')
                                .long("method")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: ParserInstallMethod = <ParserInstallMethod as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("compiler")
                                .value_name("COMPILER")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        CompilerOption,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Compiler to use")
                                .long_help(None)
                                .short('c')
                                .long("compiler")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: CompilerOption = <CompilerOption as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Compile all parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("tag")
                                .value_name("TAG")
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help(
                                    "'nvim-treesitter-parsers' tags to use.\nWill only use tags present in the changelog.\n(defaults to latest tag)",
                                )
                                .long_help(
                                    "'nvim-treesitter-parsers' tags to use.\nWill only use tags present in the changelog.\n(defaults to latest tag)\n\nSee https://github.com/KevinSilvester/nvim-treesitter-parerers",
                                )
                                .long("tag");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Compile parsers in 'wanted-parsers.txt'")
                                .long_help(None)
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Install specific parsers (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("force")
                                .value_name("FORCE")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Force install, even if parsers are already installed",
                                )
                                .long_help(None)
                                .long("force")
                                .default_value("false");
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_app
                }
            }
        }
        impl Install {
            fn select_tag(&self, changelog: &ChangeLog) -> String {
                match &self.tag {
                    Some(tag) => tag.clone(),
                    None => changelog.get_latest_tag().unwrap(),
                }
            }
            fn select_langs(&self, parsers: &Parsers) -> anyhow::Result<Vec<String>> {
                if self.all {
                    return Ok(parsers.langs.clone());
                }
                let langs = match self.wanted {
                    true => {
                        if parsers.wanted.is_none() {
                            return Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("No wanted parsers found"),
                                    );
                                    error
                                }),
                            );
                        }
                        parsers.wanted.clone().unwrap()
                    }
                    false => self.parsers.clone(),
                };
                parsers.validate_parsers(&langs)?;
                if langs.is_empty() {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("No parsers found"),
                            );
                            error
                        }),
                    );
                }
                Ok(langs)
            }
            fn cleanup(&self) -> anyhow::Result<()> {
                let destination = PATHS.ts_parsers.join(".install-tmp");
                if destination.exists() {
                    ufs::remove_all(&destination)?;
                }
                Ok(())
            }
        }
        impl Subcommand for Install {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn run<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        let compiler = &*select_compiler(&__self.compiler);
                        let mut state = State::new()?;
                        let mut parsers = Parsers::new()?;
                        let mut changelog = ChangeLog::new();
                        changelog.fetch_changelog().await?;
                        changelog.check_entry(&__self.tag)?;
                        parsers.fetch_list(&__self.tag).await?;
                        let destination = PATHS.ts_parsers.join(".install-tmp");
                        __self.cleanup()?;
                        let langs = __self.select_langs(&parsers)?;
                        let tag = __self.select_tag(&changelog);
                        let (is_installed, _) = state.check_all_installed(&langs);
                        if !__self.force && !is_installed.is_empty() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(245, 181, 61)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!(
                                                                "Parsers are already installed: {0:?}",
                                                                is_installed,
                                                            ),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                            return Ok(());
                        }
                        let msg = match __self.force {
                            true => "Force installing parser",
                            false => "Installing parser",
                        };
                        if __self.force {
                            backups_ops::create_backup(
                                &mut state,
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}-force-install", tag),
                                    );
                                    res
                                },
                            )?;
                        }
                        match __self.method {
                            ParserInstallMethod::Compile => {
                                parser_ops::check_compile_deps(compiler)?;
                                for (idx, lang) in langs.iter().enumerate() {
                                    {
                                        {
                                            ::std::io::_print(
                                                format_args!(
                                                    "{0}\n",
                                                    ::ansi_term::Colour::RGB(2, 149, 235)
                                                        .paint(
                                                            &{
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "\n{0}/{1}. {2} {3}",
                                                                        (idx + 1),
                                                                        langs.len(),
                                                                        msg,
                                                                        lang,
                                                                    ),
                                                                );
                                                                res
                                                            },
                                                        ),
                                                ),
                                            );
                                        }
                                    };
                                    let parser = parsers.get_parser(lang).unwrap();
                                    parser_ops::compile(
                                            lang,
                                            parser,
                                            compiler,
                                            &None,
                                            &destination,
                                        )
                                        .await?;
                                    state
                                        .add_parser(
                                            lang,
                                            &tag,
                                            ParserInstallMethod::Compile,
                                            parser,
                                        );
                                }
                            }
                            ParserInstallMethod::Download => {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "not yet implemented: {0}",
                                            format_args!("beep boop beep boop! I\'m a robot!"),
                                        ),
                                    );
                                };
                            }
                        }
                        ufs::copy_all(&destination, PATHS.ts_parsers.join("parsers"))?;
                        state.commit()?;
                        Ok(())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
        impl Drop for Install {
            fn drop(&mut self) {
                if self.cleanup().is_err() {
                    {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "{0}\n",
                                    ::ansi_term::Colour::RGB(245, 181, 61)
                                        .paint(
                                            &{
                                                let res = ::alloc::fmt::format(
                                                    format_args!("WARNING: Cleanup failed"),
                                                );
                                                res
                                            },
                                        ),
                                ),
                            );
                        }
                    };
                }
            }
        }
    }
    mod lock {
        use crate::{c_println, data::{parsers::Parsers, state::State}};
        use super::Subcommand;
        pub struct Lock {
            /// Lock all installed parsers
            #[clap(short, long, default_value = "false")]
            all: bool,
            /// Lock parsers in `wanted-parsers.txt`.
            /// This will only uninstall parsers that are already installed.
            ///
            /// Cannot be used with --all or [PARSERS...]
            #[clap(
                short,
                long,
                default_value = "false",
                conflicts_with_all = ["all",
                "parsers"],
                verbatim_doc_comment
            )]
            wanted: bool,
            /// Lock specific parsers (cannot be used with --all or --wanted)
            #[clap(conflicts_with = "all")]
            parsers: Vec<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Lock {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Lock",
                    "all",
                    &self.all,
                    "wanted",
                    &self.wanted,
                    "parsers",
                    &&self.parsers,
                )
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for Lock {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = Lock {
                    all: __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?,
                    wanted: __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?,
                    parsers: __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new),
                };
                ::std::result::Result::Ok(v)
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if __clap_arg_matches.contains_id("all") {
                    #[allow(non_snake_case)]
                    let all = &mut self.all;
                    *all = __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?;
                }
                if __clap_arg_matches.contains_id("wanted") {
                    #[allow(non_snake_case)]
                    let wanted = &mut self.wanted;
                    *wanted = __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?;
                }
                if __clap_arg_matches.contains_id("parsers") {
                    #[allow(non_snake_case)]
                    let parsers = &mut self.parsers;
                    *parsers = __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new);
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Args for Lock {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("Lock"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Lock")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 3usize] = [
                                        clap::Id::from("all"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Lock all installed parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Lock parsers in `wanted-parsers.txt`.\nThis will only uninstall parsers that are already installed.",
                                )
                                .long_help(
                                    "Lock parsers in `wanted-parsers.txt`.\nThis will only uninstall parsers that are already installed.\n\nCannot be used with --all or [PARSERS...]",
                                )
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Lock specific parsers (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg;
                            arg
                        });
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Lock")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 3usize] = [
                                        clap::Id::from("all"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Lock all installed parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Lock parsers in `wanted-parsers.txt`.\nThis will only uninstall parsers that are already installed.",
                                )
                                .long_help(
                                    "Lock parsers in `wanted-parsers.txt`.\nThis will only uninstall parsers that are already installed.\n\nCannot be used with --all or [PARSERS...]",
                                )
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Lock specific parsers (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_app
                }
            }
        }
        impl Lock {
            fn select_langs(
                &self,
                parsers: &Parsers,
                state: &State,
            ) -> anyhow::Result<Vec<String>> {
                if self.all {
                    return Ok(state.parsers.keys().cloned().collect());
                }
                let langs = match self.wanted {
                    true => {
                        if parsers.wanted.is_none() {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("No wanted parsers found"),
                                );
                                error
                            });
                        }
                        parsers.wanted.clone().unwrap()
                    }
                    false => self.parsers.clone(),
                };
                if langs.is_empty() {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("No parsers found"),
                        );
                        error
                    });
                }
                Ok(langs)
            }
        }
        impl Subcommand for Lock {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn run<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        let mut state = State::new()?;
                        let parsers = Parsers::new()?;
                        let langs = __self.select_langs(&parsers, &state)?;
                        let (is_installed, not_installed) = state
                            .check_all_installed(&langs);
                        if !not_installed.is_empty() {
                            return ::anyhow::__private::Err(
                                ::anyhow::Error::msg({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Parsers are not installed: {0:?}",
                                            not_installed,
                                        ),
                                    );
                                    res
                                }),
                            );
                        }
                        for (idx, lang) in is_installed.iter().enumerate() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(2, 149, 235)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}/{1}. Locking parser {2}",
                                                                (idx + 1),
                                                                is_installed.len(),
                                                                lang,
                                                            ),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                            state.lock_parser(lang);
                        }
                        state.commit()?;
                        Ok(())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    mod uninstall {
        use crate::{
            c_println, data::{parsers::Parsers, state::State},
            ops::{backups_ops, parser_ops},
            utils::PATHS,
        };
        use super::Subcommand;
        pub struct Uninstall {
            /// Uninstall all installed parsers
            #[clap(short, long, default_value = "false")]
            all: bool,
            /// Uninstall parsers in 'wanted-parsers.txt'.
            /// This will only uninstall parsers that are already installed.
            ///
            /// Cannot be used with --all or [PARSERS...]
            #[clap(
                short,
                long,
                default_value = "false",
                conflicts_with_all = ["all",
                "parsers"],
                verbatim_doc_comment
            )]
            wanted: bool,
            /// Uninstall specific parsers (cannot be used with --all or --wanted)
            #[clap(conflicts_with = "all")]
            parsers: Vec<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Uninstall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Uninstall",
                    "all",
                    &self.all,
                    "wanted",
                    &self.wanted,
                    "parsers",
                    &&self.parsers,
                )
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for Uninstall {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = Uninstall {
                    all: __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?,
                    wanted: __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?,
                    parsers: __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new),
                };
                ::std::result::Result::Ok(v)
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if __clap_arg_matches.contains_id("all") {
                    #[allow(non_snake_case)]
                    let all = &mut self.all;
                    *all = __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?;
                }
                if __clap_arg_matches.contains_id("wanted") {
                    #[allow(non_snake_case)]
                    let wanted = &mut self.wanted;
                    *wanted = __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?;
                }
                if __clap_arg_matches.contains_id("parsers") {
                    #[allow(non_snake_case)]
                    let parsers = &mut self.parsers;
                    *parsers = __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new);
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Args for Uninstall {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("Uninstall"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Uninstall")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 3usize] = [
                                        clap::Id::from("all"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Uninstall all installed parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Uninstall parsers in 'wanted-parsers.txt'.\nThis will only uninstall parsers that are already installed.",
                                )
                                .long_help(
                                    "Uninstall parsers in 'wanted-parsers.txt'.\nThis will only uninstall parsers that are already installed.\n\nCannot be used with --all or [PARSERS...]",
                                )
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Uninstall specific parsers (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg;
                            arg
                        });
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Uninstall")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 3usize] = [
                                        clap::Id::from("all"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Uninstall all installed parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Uninstall parsers in 'wanted-parsers.txt'.\nThis will only uninstall parsers that are already installed.",
                                )
                                .long_help(
                                    "Uninstall parsers in 'wanted-parsers.txt'.\nThis will only uninstall parsers that are already installed.\n\nCannot be used with --all or [PARSERS...]",
                                )
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Uninstall specific parsers (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_app
                }
            }
        }
        impl Uninstall {
            fn select_langs(
                &self,
                parsers: &Parsers,
                state: &State,
            ) -> anyhow::Result<Vec<String>> {
                if self.all {
                    return Ok(state.parsers.keys().cloned().collect());
                }
                let langs = match self.wanted {
                    true => {
                        if parsers.wanted.is_none() {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("No wanted parsers found"),
                                );
                                error
                            });
                        }
                        parsers.wanted.clone().unwrap()
                    }
                    false => self.parsers.clone(),
                };
                if langs.is_empty() {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("No parsers found"),
                        );
                        error
                    });
                }
                Ok(langs)
            }
        }
        impl Subcommand for Uninstall {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn run<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        let mut state = State::new()?;
                        let parsers = Parsers::new()?;
                        let langs = __self.select_langs(&parsers, &state)?;
                        let (is_installed, not_installed) = state
                            .check_all_installed(&langs);
                        let to_uninstall = &is_installed
                            .iter()
                            .filter(|lang| !state.is_locked(lang))
                            .collect::<Vec<_>>();
                        let is_locked = &is_installed
                            .iter()
                            .filter(|lang| state.is_locked(lang))
                            .collect::<Vec<_>>();
                        if !is_locked.is_empty() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(245, 181, 61)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!("Parsers are locked: {0:?}", is_locked),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                        }
                        if !not_installed.is_empty() {
                            return ::anyhow::__private::Err(
                                ::anyhow::Error::msg({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Parsers are not installed: {0:?}",
                                            not_installed,
                                        ),
                                    );
                                    res
                                }),
                            );
                        }
                        if to_uninstall.is_empty() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(2, 149, 235)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!("No parsers to uninstall!"),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                            return Ok(());
                        }
                        backups_ops::create_backup(&mut state, "uninstall")?;
                        for (idx, lang) in is_installed.iter().enumerate() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(2, 149, 235)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}/{1}. Uninstalling parser {2}",
                                                                (idx + 1),
                                                                is_installed.len(),
                                                                lang,
                                                            ),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                            parser_ops::uninstall(
                                lang,
                                &PATHS.ts_parsers.join("parsers"),
                            )?;
                            state.remove_parser(lang);
                        }
                        state.commit()?;
                        Ok(())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    mod unlock {
        use crate::{c_println, data::{parsers::Parsers, state::State}};
        use super::Subcommand;
        pub struct Unlock {
            /// Unlock all installed parsers
            #[clap(short, long, default_value = "false")]
            all: bool,
            /// Unlock parsers in `wanted-parsers.txt`.
            /// This will only uninstall parsers that are already installed.
            ///
            /// Cannot be used with --all or [PARSERS...]
            #[clap(
                short,
                long,
                default_value = "false",
                conflicts_with_all = ["all",
                "parsers"],
                verbatim_doc_comment
            )]
            wanted: bool,
            /// Unlock specific parsers (cannot be used with --all or --wanted)
            #[clap(conflicts_with = "all")]
            parsers: Vec<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Unlock {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Unlock",
                    "all",
                    &self.all,
                    "wanted",
                    &self.wanted,
                    "parsers",
                    &&self.parsers,
                )
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for Unlock {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = Unlock {
                    all: __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?,
                    wanted: __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?,
                    parsers: __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new),
                };
                ::std::result::Result::Ok(v)
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if __clap_arg_matches.contains_id("all") {
                    #[allow(non_snake_case)]
                    let all = &mut self.all;
                    *all = __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?;
                }
                if __clap_arg_matches.contains_id("wanted") {
                    #[allow(non_snake_case)]
                    let wanted = &mut self.wanted;
                    *wanted = __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?;
                }
                if __clap_arg_matches.contains_id("parsers") {
                    #[allow(non_snake_case)]
                    let parsers = &mut self.parsers;
                    *parsers = __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new);
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Args for Unlock {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("Unlock"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Unlock")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 3usize] = [
                                        clap::Id::from("all"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Unlock all installed parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Unlock parsers in `wanted-parsers.txt`.\nThis will only uninstall parsers that are already installed.",
                                )
                                .long_help(
                                    "Unlock parsers in `wanted-parsers.txt`.\nThis will only uninstall parsers that are already installed.\n\nCannot be used with --all or [PARSERS...]",
                                )
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Unlock specific parsers (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg;
                            arg
                        });
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Unlock")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 3usize] = [
                                        clap::Id::from("all"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Unlock all installed parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Unlock parsers in `wanted-parsers.txt`.\nThis will only uninstall parsers that are already installed.",
                                )
                                .long_help(
                                    "Unlock parsers in `wanted-parsers.txt`.\nThis will only uninstall parsers that are already installed.\n\nCannot be used with --all or [PARSERS...]",
                                )
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Unlock specific parsers (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_app
                }
            }
        }
        impl Unlock {
            fn select_langs(
                &self,
                parsers: &Parsers,
                state: &State,
            ) -> anyhow::Result<Vec<String>> {
                if self.all {
                    return Ok(state.parsers.keys().cloned().collect());
                }
                let langs = match self.wanted {
                    true => {
                        if parsers.wanted.is_none() {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!("No wanted parsers found"),
                                );
                                error
                            });
                        }
                        parsers.wanted.clone().unwrap()
                    }
                    false => self.parsers.clone(),
                };
                if langs.is_empty() {
                    return ::anyhow::__private::Err({
                        let error = ::anyhow::__private::format_err(
                            format_args!("No parsers found"),
                        );
                        error
                    });
                }
                Ok(langs)
            }
        }
        impl Subcommand for Unlock {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn run<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        let mut state = State::new()?;
                        let parsers = Parsers::new()?;
                        let langs = __self.select_langs(&parsers, &state)?;
                        let (is_installed, not_installed) = state
                            .check_all_installed(&langs);
                        if !not_installed.is_empty() {
                            return ::anyhow::__private::Err(
                                ::anyhow::Error::msg({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Parsers are not installed: {0:?}",
                                            not_installed,
                                        ),
                                    );
                                    res
                                }),
                            );
                        }
                        for (idx, lang) in is_installed.iter().enumerate() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(2, 149, 235)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!(
                                                                "\n{0}/{1}. Unlocking parser {2}",
                                                                (idx + 1),
                                                                is_installed.len(),
                                                                lang,
                                                            ),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                            state.unlock_parser(lang);
                        }
                        state.commit()?;
                        Ok(())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    mod update {
        use crate::{
            c_println, compiler::{select_compiler, CompilerOption},
            data::{
                changelog::ChangeLog, parsers::Parsers,
                state::{ParserInstallMethod, State},
            },
            ops::{backups_ops, parser_ops},
            utils::{fs as ufs, PATHS},
        };
        use super::Subcommand;
        pub struct Update {
            /// Installation method
            #[clap(short, long, default_value_t, value_enum)]
            method: ParserInstallMethod,
            /// Compiler to use
            #[clap(short, long, default_value_t, value_enum)]
            compiler: CompilerOption,
            /// Update all installed parsers
            #[clap(short, long, default_value = "false")]
            all: bool,
            /// Update parsers in 'wanted-parsers.txt'.
            /// This will only update parsers that are already installed.
            ///
            /// Cannot be used with --all or [PARSERS...]
            #[clap(
                short,
                long,
                default_value = "false",
                conflicts_with_all = ["all",
                "parsers"],
                verbatim_doc_comment
            )]
            wanted: bool,
            /// Update specific (cannot be used with --all or --wanted)
            #[clap(conflicts_with = "all")]
            parsers: Vec<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Update {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Update",
                    "method",
                    &self.method,
                    "compiler",
                    &self.compiler,
                    "all",
                    &self.all,
                    "wanted",
                    &self.wanted,
                    "parsers",
                    &&self.parsers,
                )
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::FromArgMatches for Update {
            fn from_arg_matches(
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn from_arg_matches_mut(
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<Self, clap::Error> {
                #![allow(deprecated)]
                let v = Update {
                    method: __clap_arg_matches
                        .remove_one::<ParserInstallMethod>("method")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: method",
                        ))?,
                    compiler: __clap_arg_matches
                        .remove_one::<CompilerOption>("compiler")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: compiler",
                        ))?,
                    all: __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?,
                    wanted: __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?,
                    parsers: __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new),
                };
                ::std::result::Result::Ok(v)
            }
            fn update_from_arg_matches(
                &mut self,
                __clap_arg_matches: &clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
            }
            fn update_from_arg_matches_mut(
                &mut self,
                __clap_arg_matches: &mut clap::ArgMatches,
            ) -> ::std::result::Result<(), clap::Error> {
                #![allow(deprecated)]
                if __clap_arg_matches.contains_id("method") {
                    #[allow(non_snake_case)]
                    let method = &mut self.method;
                    *method = __clap_arg_matches
                        .remove_one::<ParserInstallMethod>("method")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: method",
                        ))?;
                }
                if __clap_arg_matches.contains_id("compiler") {
                    #[allow(non_snake_case)]
                    let compiler = &mut self.compiler;
                    *compiler = __clap_arg_matches
                        .remove_one::<CompilerOption>("compiler")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: compiler",
                        ))?;
                }
                if __clap_arg_matches.contains_id("all") {
                    #[allow(non_snake_case)]
                    let all = &mut self.all;
                    *all = __clap_arg_matches
                        .remove_one::<bool>("all")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: all",
                        ))?;
                }
                if __clap_arg_matches.contains_id("wanted") {
                    #[allow(non_snake_case)]
                    let wanted = &mut self.wanted;
                    *wanted = __clap_arg_matches
                        .remove_one::<bool>("wanted")
                        .ok_or_else(|| clap::Error::raw(
                            clap::error::ErrorKind::MissingRequiredArgument,
                            "The following required argument was not provided: wanted",
                        ))?;
                }
                if __clap_arg_matches.contains_id("parsers") {
                    #[allow(non_snake_case)]
                    let parsers = &mut self.parsers;
                    *parsers = __clap_arg_matches
                        .remove_many::<String>("parsers")
                        .map(|v| v.collect::<Vec<_>>())
                        .unwrap_or_else(Vec::new);
                }
                ::std::result::Result::Ok(())
            }
        }
        #[allow(
            dead_code,
            unreachable_code,
            unused_variables,
            unused_braces,
            unused_qualifications,
        )]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo,
            clippy::suspicious_else_formatting,
            clippy::almost_swapped,
            clippy::redundant_locals,
        )]
        #[automatically_derived]
        impl clap::Args for Update {
            fn group_id() -> Option<clap::Id> {
                Some(clap::Id::from("Update"))
            }
            fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Update")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 5usize] = [
                                        clap::Id::from("method"),
                                        clap::Id::from("compiler"),
                                        clap::Id::from("all"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("method")
                                .value_name("METHOD")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        ParserInstallMethod,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Installation method")
                                .long_help(None)
                                .short('m')
                                .long("method")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: ParserInstallMethod = <ParserInstallMethod as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("compiler")
                                .value_name("COMPILER")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        CompilerOption,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Compiler to use")
                                .long_help(None)
                                .short('c')
                                .long("compiler")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: CompilerOption = <CompilerOption as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Update all installed parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Update parsers in 'wanted-parsers.txt'.\nThis will only update parsers that are already installed.",
                                )
                                .long_help(
                                    "Update parsers in 'wanted-parsers.txt'.\nThis will only update parsers that are already installed.\n\nCannot be used with --all or [PARSERS...]",
                                )
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg;
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Update specific (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg;
                            arg
                        });
                    __clap_app
                }
            }
            fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
                {
                    let __clap_app = __clap_app
                        .group(
                            clap::ArgGroup::new("Update")
                                .multiple(true)
                                .args({
                                    let members: [clap::Id; 5usize] = [
                                        clap::Id::from("method"),
                                        clap::Id::from("compiler"),
                                        clap::Id::from("all"),
                                        clap::Id::from("wanted"),
                                        clap::Id::from("parsers"),
                                    ];
                                    members
                                }),
                        );
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("method")
                                .value_name("METHOD")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        ParserInstallMethod,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Installation method")
                                .long_help(None)
                                .short('m')
                                .long("method")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: ParserInstallMethod = <ParserInstallMethod as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("compiler")
                                .value_name("COMPILER")
                                .required(false && clap::ArgAction::Set.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        CompilerOption,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Set);
                            let arg = arg
                                .help("Compiler to use")
                                .long_help(None)
                                .short('c')
                                .long("compiler")
                                .default_value({
                                    static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                    let s = DEFAULT_VALUE
                                        .get_or_init(|| {
                                            let val: CompilerOption = <CompilerOption as ::std::default::Default>::default();
                                            clap::ValueEnum::to_possible_value(&val)
                                                .unwrap()
                                                .get_name()
                                                .to_owned()
                                        });
                                    let s: &'static str = &*s;
                                    s
                                });
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("all")
                                .value_name("ALL")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help("Update all installed parsers")
                                .long_help(None)
                                .short('a')
                                .long("all")
                                .default_value("false");
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("wanted")
                                .value_name("WANTED")
                                .required(false && clap::ArgAction::SetTrue.takes_values())
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        bool,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::SetTrue);
                            let arg = arg
                                .help(
                                    "Update parsers in 'wanted-parsers.txt'.\nThis will only update parsers that are already installed.",
                                )
                                .long_help(
                                    "Update parsers in 'wanted-parsers.txt'.\nThis will only update parsers that are already installed.\n\nCannot be used with --all or [PARSERS...]",
                                )
                                .short('w')
                                .long("wanted")
                                .default_value("false")
                                .conflicts_with_all(["all", "parsers"]);
                            let arg = arg.required(false);
                            arg
                        });
                    let __clap_app = __clap_app
                        .arg({
                            #[allow(deprecated)]
                            let arg = clap::Arg::new("parsers")
                                .value_name("PARSERS")
                                .num_args(1..)
                                .value_parser({
                                    use ::clap_builder::builder::via_prelude::*;
                                    let auto = ::clap_builder::builder::_AutoValueParser::<
                                        String,
                                    >::new();
                                    (&&&&&&auto).value_parser()
                                })
                                .action(clap::ArgAction::Append);
                            let arg = arg
                                .help(
                                    "Update specific (cannot be used with --all or --wanted)",
                                )
                                .long_help(None)
                                .conflicts_with("all");
                            let arg = arg.required(false);
                            arg
                        });
                    __clap_app
                }
            }
        }
        impl Update {
            fn select_langs(&self, parsers: &Parsers) -> anyhow::Result<Vec<String>> {
                if self.all {
                    return Ok(parsers.langs.clone());
                }
                let langs = match self.wanted {
                    true => {
                        if parsers.wanted.is_none() {
                            {
                                let red = ::anstyle::RgbColor(235, 66, 66).on_default();
                                {
                                    ::std::io::_eprint(
                                        format_args!(
                                            "{1}{0}{1:#}\n",
                                            {
                                                let res = ::alloc::fmt::format(
                                                    format_args!("No wanted parsers found"),
                                                );
                                                res
                                            },
                                            red,
                                        ),
                                    );
                                }
                            };
                            return Err(
                                ::anyhow::__private::must_use({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("No wanted parsers found"),
                                    );
                                    error
                                }),
                            );
                        }
                        parsers.wanted.clone().unwrap()
                    }
                    false => self.parsers.clone(),
                };
                parsers.validate_parsers(&langs)?;
                if langs.is_empty() {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                format_args!("No parsers found"),
                            );
                            error
                        }),
                    );
                }
                Ok(langs)
            }
            fn cleanup(&self) -> anyhow::Result<()> {
                let destination = PATHS.ts_parsers.join(".update-tmp");
                if destination.exists() {
                    ufs::remove_all(&destination)?;
                }
                Ok(())
            }
        }
        impl Subcommand for Update {
            #[allow(
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn run<'life0, 'async_trait>(
                &'life0 self,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = anyhow::Result<()>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        anyhow::Result<()>,
                    > {
                        return __ret;
                    }
                    let __self = self;
                    let __ret: anyhow::Result<()> = {
                        let compiler = &*select_compiler(&__self.compiler);
                        let mut state = State::new()?;
                        let mut parsers = Parsers::new()?;
                        let mut changelog = ChangeLog::new();
                        changelog.fetch_changelog().await?;
                        changelog.check_entry(&None)?;
                        parsers.fetch_list(&None).await?;
                        let destination = PATHS.ts_parsers.join(".update-tmp");
                        __self.cleanup()?;
                        let langs = __self.select_langs(&parsers)?;
                        let tag = changelog.get_latest_tag().unwrap();
                        let (is_installed, not_installed) = state
                            .check_all_installed(&langs);
                        if is_installed.is_empty() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(245, 181, 61)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!("Selected parsers are not installed!"),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                            return Ok(());
                        }
                        if !not_installed.is_empty() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(245, 181, 61)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!(
                                                                "Parsers are not installed: {0:?}",
                                                                not_installed,
                                                            ),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                        }
                        let is_locked = &is_installed
                            .iter()
                            .filter(|lang| state.is_locked(lang))
                            .collect::<Vec<_>>();
                        let up_to_date = &is_installed
                            .iter()
                            .filter(|lang| state.is_up_to_date(lang, &tag))
                            .collect::<Vec<_>>();
                        let to_update = &is_installed
                            .iter()
                            .filter(|lang| {
                                !state.is_up_to_date(lang, &tag) && !state.is_locked(lang)
                            })
                            .collect::<Vec<_>>();
                        if !is_locked.is_empty() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(245, 181, 61)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!("Parsers are locked: {0:?}", is_locked),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                        }
                        if !up_to_date.is_empty() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(2, 149, 235)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!(
                                                                "Parsers are already up-to-date: {0:?}",
                                                                up_to_date,
                                                            ),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                        }
                        if to_update.is_empty() {
                            {
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "{0}\n",
                                            ::ansi_term::Colour::RGB(2, 149, 235)
                                                .paint(
                                                    &{
                                                        let res = ::alloc::fmt::format(
                                                            format_args!("No parsers to update!"),
                                                        );
                                                        res
                                                    },
                                                ),
                                        ),
                                    );
                                }
                            };
                            return Ok(());
                        }
                        backups_ops::create_backup(
                            &mut state,
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}-update", tag),
                                );
                                res
                            },
                        )?;
                        match __self.method {
                            ParserInstallMethod::Compile => {
                                parser_ops::check_compile_deps(compiler)?;
                                for (idx, lang) in to_update.iter().enumerate() {
                                    {
                                        {
                                            ::std::io::_print(
                                                format_args!(
                                                    "{0}\n",
                                                    ::ansi_term::Colour::RGB(2, 149, 235)
                                                        .paint(
                                                            &{
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "\n{0}/{1}. Updating parser {2}",
                                                                        (idx + 1),
                                                                        to_update.len(),
                                                                        lang,
                                                                    ),
                                                                );
                                                                res
                                                            },
                                                        ),
                                                ),
                                            );
                                        }
                                    };
                                    let parser = parsers.get_parser(lang).unwrap();
                                    parser_ops::compile(
                                            lang,
                                            parser,
                                            compiler,
                                            &None,
                                            &destination,
                                        )
                                        .await?;
                                    state
                                        .update_parser(
                                            lang,
                                            &tag,
                                            ParserInstallMethod::Compile,
                                            parser,
                                        );
                                }
                            }
                            ParserInstallMethod::Download => {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "not yet implemented: {0}",
                                            format_args!("beep boop beep boop! I\'m a robot!"),
                                        ),
                                    );
                                };
                            }
                        }
                        ufs::copy_all(&destination, PATHS.ts_parsers.join("parsers"))?;
                        state.commit()?;
                        Ok(())
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
        impl Drop for Update {
            fn drop(&mut self) {
                if self.cleanup().is_err() {
                    {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "{0}\n",
                                    ::ansi_term::Colour::RGB(245, 181, 61)
                                        .paint(
                                            &{
                                                let res = ::alloc::fmt::format(
                                                    format_args!("WARNING: Cleanup failed"),
                                                );
                                                res
                                            },
                                        ),
                                ),
                            );
                        }
                    };
                }
            }
        }
    }
    pub trait Subcommand {
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn run<'life0, 'async_trait>(
            &'life0 self,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                    Output = anyhow::Result<()>,
                > + ::core::marker::Send + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait;
    }
    pub use self::backups::Backups;
    pub use self::compile::Compile;
    pub use self::install::Install;
    pub use self::lock::Lock;
    pub use self::uninstall::Uninstall;
    pub use self::unlock::Unlock;
    pub use self::update::Update;
}
mod utils {
    mod paths {
        use std::path::PathBuf;
        use lazy_static::lazy_static;
        pub struct Paths {
            pub wanted_parsers: PathBuf,
            pub ts_parsers: PathBuf,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Paths {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Paths",
                    "wanted_parsers",
                    &self.wanted_parsers,
                    "ts_parsers",
                    &&self.ts_parsers,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Paths {
            #[inline]
            fn clone(&self) -> Paths {
                Paths {
                    wanted_parsers: ::core::clone::Clone::clone(&self.wanted_parsers),
                    ts_parsers: ::core::clone::Clone::clone(&self.ts_parsers),
                }
            }
        }
        #[allow(unused)]
        #[cfg(unix)]
        const NVIM_DATA_DIR: &str = "nvim";
        impl Paths {
            #[cfg(not(test))]
            pub fn new() -> Self {
                use cfg_if::cfg_if;
                let local_config_dir = dirs::home_dir().unwrap().join(".config");
                let local_data_dir = dirs::home_dir()
                    .unwrap()
                    .join(".local")
                    .join("share");
                let wanted_parsers = match std::env::var("TS_PARSERS_WANTED_PARSERS")
                    .ok()
                {
                    Some(dir) => PathBuf::from(&dir),
                    None => local_config_dir.join("nvim").join("wanted-parsers.txt"),
                };
                let ts_parsers = match std::env::var("TS_PARSERS_DATA").ok() {
                    Some(dir) => PathBuf::from(&dir),
                    None => local_data_dir.join(NVIM_DATA_DIR).join("ts-parsers"),
                };
                Self { wanted_parsers, ts_parsers }
            }
        }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct PATHS {
            __private_field: (),
        }
        #[doc(hidden)]
        pub static PATHS: PATHS = PATHS { __private_field: () };
        impl ::lazy_static::__Deref for PATHS {
            type Target = Paths;
            fn deref(&self) -> &Paths {
                #[inline(always)]
                fn __static_ref_initialize() -> Paths {
                    Paths::new()
                }
                #[inline(always)]
                fn __stability() -> &'static Paths {
                    static LAZY: ::lazy_static::lazy::Lazy<Paths> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for PATHS {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
    }
    mod renderer {
        use std::collections::VecDeque;
        use std::io::Write;
        use crossterm::{cursor, execute, terminal};
        pub struct Renderer<W> {
            is_first_render: bool,
            line_count: usize,
            prev_line_count: usize,
            stdout: W,
        }
        #[automatically_derived]
        impl<W: ::core::fmt::Debug> ::core::fmt::Debug for Renderer<W> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Renderer",
                    "is_first_render",
                    &self.is_first_render,
                    "line_count",
                    &self.line_count,
                    "prev_line_count",
                    &self.prev_line_count,
                    "stdout",
                    &&self.stdout,
                )
            }
        }
        impl<W: Write> Renderer<W> {
            pub fn new(writer: W) -> Self {
                Self {
                    is_first_render: true,
                    line_count: 0,
                    prev_line_count: 0,
                    stdout: writer,
                }
            }
            fn clear_ouput(&mut self) -> anyhow::Result<()> {
                if self.prev_line_count == 0 {
                    return Ok(());
                }
                {
                    use ::std::io::Write;
                    {
                        use ::std::io::Write;
                        Ok(self.stdout.by_ref())
                            .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                writer,
                                cursor::MoveToPreviousLine(self.prev_line_count as u16),
                            ))
                            .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                writer,
                                terminal::Clear(terminal::ClearType::FromCursorDown),
                            ))
                            .map(|_| ())
                    }
                        .and_then(|()| { ::std::io::Write::flush(self.stdout.by_ref()) })
                }?;
                Ok(())
            }
            pub fn clear_output_final(&mut self) -> anyhow::Result<()> {
                {
                    use ::std::io::Write;
                    {
                        use ::std::io::Write;
                        Ok(self.stdout.by_ref())
                            .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                writer,
                                cursor::MoveToPreviousLine(self.prev_line_count as u16 + 1),
                            ))
                            .and_then(|writer| ::crossterm::QueueableCommand::queue(
                                writer,
                                terminal::Clear(terminal::ClearType::FromCursorDown),
                            ))
                            .map(|_| ())
                    }
                        .and_then(|()| { ::std::io::Write::flush(self.stdout.by_ref()) })
                }?;
                Ok(())
            }
            pub fn render_queue(
                &mut self,
                queue: &VecDeque<String>,
            ) -> anyhow::Result<()> {
                self.line_count = queue.len();
                if self.is_first_render {
                    self.is_first_render = false;
                } else {
                    self.clear_ouput()?;
                }
                let width = match terminal::size() {
                    Ok((w, _)) if w > 0 => w,
                    _ => 100,
                };
                for line in queue {
                    let mut l = line.clone();
                    l.truncate(width as usize);
                    (&mut self.stdout).write_fmt(format_args!("{0}\n", l))?;
                }
                self.prev_line_count = self.line_count;
                Ok(())
            }
        }
    }
    pub mod archives {
        use std::{fs::File, io::BufReader, path::Path};
        use bzip2::{read::BzDecoder, write::BzEncoder, Compression};
        use flate2::read::GzDecoder;
        use tar::{Archive, Builder};
        use super::PATHS;
        pub fn extract_tar_gz(
            archive_path: &Path,
            extract_dir: &Path,
        ) -> anyhow::Result<()> {
            let tar_file = File::open(archive_path)?;
            let buf_reader = BufReader::new(tar_file);
            let gz_decoder = GzDecoder::new(buf_reader);
            let mut tar_archive = Archive::new(gz_decoder);
            tar_archive.unpack(extract_dir)?;
            Ok(())
        }
        pub fn extract_tar_bz2(
            archive_path: &Path,
            extract_dir: &Path,
        ) -> anyhow::Result<()> {
            let tar_file = File::open(archive_path)?;
            let buf_reader = BufReader::new(tar_file);
            let bz2_decoder = BzDecoder::new(buf_reader);
            let mut tar_archive = Archive::new(bz2_decoder);
            tar_archive.unpack(extract_dir)?;
            Ok(())
        }
        pub fn create_tar_bz2(
            ouput: &Path,
            input_paths: &[&Path],
        ) -> anyhow::Result<()> {
            let tar_bz2_file = File::create(ouput)?;
            let bz2_encoder = BzEncoder::new(tar_bz2_file, Compression::best());
            let mut tar_archive = Builder::new(bz2_encoder);
            for path in input_paths {
                append_to_archive(&mut tar_archive, path)?;
            }
            Ok(())
        }
        fn append_to_archive(
            tar_archive: &mut Builder<BzEncoder<File>>,
            path: &Path,
        ) -> anyhow::Result<()> {
            if path.is_file() {
                let mut file = File::open(path)?;
                let name = path.strip_prefix(&PATHS.ts_parsers).unwrap();
                tar_archive.append_file(name, &mut file)?;
                return Ok(());
            }
            for entry in path.read_dir()? {
                let entry = entry?;
                let path = entry.path();
                append_to_archive(tar_archive, &path)?;
            }
            Ok(())
        }
    }
    pub mod command {
        use std::{collections::VecDeque, path::Path, println, process::Stdio};
        use ansi_term::Colour;
        use lazy_static::lazy_static;
        use strip_ansi_escapes::strip_str;
        use tokio::process::Command;
        use tokio_process_stream::{Item, ProcessLineStream};
        use tokio_stream::StreamExt;
        use crate::utils::renderer::Renderer;
        const MAX_LINES: usize = 20;
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct TURQUOISE {
            __private_field: (),
        }
        #[doc(hidden)]
        static TURQUOISE: TURQUOISE = TURQUOISE { __private_field: () };
        impl ::lazy_static::__Deref for TURQUOISE {
            type Target = Colour;
            fn deref(&self) -> &Colour {
                #[inline(always)]
                fn __static_ref_initialize() -> Colour {
                    Colour::RGB(66, 242, 245)
                }
                #[inline(always)]
                fn __stability() -> &'static Colour {
                    static LAZY: ::lazy_static::lazy::Lazy<Colour> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for TURQUOISE {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct BLUE {
            __private_field: (),
        }
        #[doc(hidden)]
        static BLUE: BLUE = BLUE { __private_field: () };
        impl ::lazy_static::__Deref for BLUE {
            type Target = Colour;
            fn deref(&self) -> &Colour {
                #[inline(always)]
                fn __static_ref_initialize() -> Colour {
                    Colour::RGB(2, 149, 235)
                }
                #[inline(always)]
                fn __stability() -> &'static Colour {
                    static LAZY: ::lazy_static::lazy::Lazy<Colour> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for BLUE {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct RED {
            __private_field: (),
        }
        #[doc(hidden)]
        static RED: RED = RED { __private_field: () };
        impl ::lazy_static::__Deref for RED {
            type Target = Colour;
            fn deref(&self) -> &Colour {
                #[inline(always)]
                fn __static_ref_initialize() -> Colour {
                    Colour::RGB(235, 66, 66)
                }
                #[inline(always)]
                fn __stability() -> &'static Colour {
                    static LAZY: ::lazy_static::lazy::Lazy<Colour> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for RED {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct GREEN {
            __private_field: (),
        }
        #[doc(hidden)]
        static GREEN: GREEN = GREEN { __private_field: () };
        impl ::lazy_static::__Deref for GREEN {
            type Target = Colour;
            fn deref(&self) -> &Colour {
                #[inline(always)]
                fn __static_ref_initialize() -> Colour {
                    Colour::RGB(57, 219, 57)
                }
                #[inline(always)]
                fn __stability() -> &'static Colour {
                    static LAZY: ::lazy_static::lazy::Lazy<Colour> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for GREEN {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        pub fn check_exists(command: &str) -> anyhow::Result<()> {
            match std::process::Command::new(command).stdout(Stdio::null()).output() {
                Ok(_) => Ok(()),
                Err(error) => {
                    match error.kind() {
                        std::io::ErrorKind::NotFound
                        | std::io::ErrorKind::PermissionDenied => {
                            return ::anyhow::__private::Err({
                                let error = ::anyhow::__private::format_err(
                                    format_args!(
                                        "Command \'{0}\' not found: {1}",
                                        command,
                                        error,
                                    ),
                                );
                                error
                            });
                        }
                        _ => Ok(()),
                    }
                }
            }
        }
        pub async fn run(
            name: &str,
            args: &[&str],
            cwd: Option<impl AsRef<Path>>,
        ) -> anyhow::Result<bool> {
            let mut command = Command::new(name);
            command.args(args);
            if let Some(cwd) = cwd {
                command.current_dir(cwd);
            }
            let mut renderer = Renderer::new(std::io::stdout());
            let mut out_queue: VecDeque<String> = VecDeque::with_capacity(MAX_LINES);
            let mut is_finished = false;
            let mut failed = false;
            {
                ::std::io::_print(
                    format_args!(
                        "{0} {1} {2} {3}\n",
                        TURQUOISE.paint("=>"),
                        name,
                        args.join(" "),
                        BLUE.paint("(running...)"),
                    ),
                );
            };
            let mut procstream = ProcessLineStream::try_from(command)?;
            for _ in 0..MAX_LINES {
                if let Some(item) = procstream.next().await {
                    match item {
                        Item::Done(status) => {
                            is_finished = true;
                            failed = !status.unwrap().success();
                            break;
                        }
                        Item::Stdout(out) => {
                            out_queue
                                .push_back({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "   {0} {1}",
                                            BLUE.paint("=>"),
                                            strip_str(&out),
                                        ),
                                    );
                                    res
                                });
                        }
                        Item::Stderr(err) => {
                            out_queue
                                .push_back({
                                    let res = ::alloc::fmt::format(
                                        format_args!("   {0} {1}", RED.paint("=>"), strip_str(&err)),
                                    );
                                    res
                                });
                        }
                    }
                    renderer.render_queue(&out_queue)?;
                } else {
                    is_finished = true;
                    break;
                }
            }
            if !is_finished {
                while let Some(item) = procstream.next().await {
                    out_queue.pop_front();
                    match item {
                        Item::Done(status) => {
                            failed = !status.unwrap().success();
                            break;
                        }
                        Item::Stdout(out) => {
                            out_queue
                                .push_back({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "   {0} {1}",
                                            BLUE.paint("=>"),
                                            strip_str(&out),
                                        ),
                                    );
                                    res
                                });
                        }
                        Item::Stderr(err) => {
                            out_queue
                                .push_back({
                                    let res = ::alloc::fmt::format(
                                        format_args!("   {0} {1}", RED.paint("=>"), strip_str(&err)),
                                    );
                                    res
                                });
                        }
                    }
                    renderer.render_queue(&out_queue)?;
                }
            }
            renderer.clear_output_final()?;
            if failed {
                {
                    ::std::io::_eprint(
                        format_args!(
                            "{0} {1} {2} {3}\n",
                            RED.paint("=>"),
                            name,
                            args.join(" "),
                            RED.paint("(failed!)"),
                        ),
                    );
                };
                return Ok(false);
            }
            {
                ::std::io::_print(
                    format_args!(
                        "{0} {1} {2} {3}\n",
                        TURQUOISE.paint("=>"),
                        name,
                        args.join(" "),
                        GREEN.paint("(complete!)"),
                    ),
                );
            };
            Ok(true)
        }
    }
    pub mod fs {
        use std::{fs, path::Path};
        pub fn copy_all(
            src: impl AsRef<Path>,
            dst: impl AsRef<Path>,
        ) -> anyhow::Result<()> {
            fs::create_dir_all(&dst)?;
            for entry in fs::read_dir(src)? {
                let entry = entry?;
                let ty = entry.file_type()?;
                if ty.is_dir() {
                    copy_all(&entry.path(), dst.as_ref().join(entry.file_name()))?;
                } else {
                    fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
                }
            }
            Ok(())
        }
        pub fn remove_all(path: impl AsRef<Path>) -> anyhow::Result<()> {
            if path.as_ref().is_file() {
                std::fs::remove_file(path)?;
                return Ok(());
            }
            for entry in path.as_ref().read_dir()? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    remove_all(&path)?;
                } else {
                    std::fs::remove_file(&path)?;
                }
            }
            std::fs::remove_dir(path)?;
            Ok(())
        }
    }
    pub mod http {
        use std::fs::File;
        use std::io::Cursor;
        use std::path::Path;
        use lazy_static::lazy_static;
        use reqwest::redirect::Policy;
        use reqwest::Client;
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct CLIENT {
            __private_field: (),
        }
        #[doc(hidden)]
        static CLIENT: CLIENT = CLIENT { __private_field: () };
        impl ::lazy_static::__Deref for CLIENT {
            type Target = Client;
            fn deref(&self) -> &Client {
                #[inline(always)]
                fn __static_ref_initialize() -> Client {
                    Client::builder()
                        .redirect(Policy::limited(5))
                        .use_rustls_tls()
                        .user_agent("ts-parsers/0.0.0")
                        .build()
                        .unwrap_or_else(|err| {
                            ::core::panicking::panic_fmt(
                                format_args!("Failed to create HTTP client: {0}", err),
                            );
                        })
                }
                #[inline(always)]
                fn __stability() -> &'static Client {
                    static LAZY: ::lazy_static::lazy::Lazy<Client> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for CLIENT {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        /// reference: https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
        pub async fn download_file(
            url: &str,
            destination: impl AsRef<Path>,
        ) -> anyhow::Result<()> {
            let response = CLIENT.get(url).send().await?;
            if !response.status().is_success() {
                return ::anyhow::__private::Err(
                    ::anyhow::Error::msg({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Failed to download file: {0}",
                                response.status(),
                            ),
                        );
                        res
                    }),
                );
            }
            let mut file = File::create(destination)?;
            let mut content = Cursor::new(response.bytes().await?);
            std::io::copy(&mut content, &mut file)?;
            Ok(())
        }
    }
    use anstyle::{RgbColor, Style};
    pub use self::paths::PATHS;
    pub const RED: Style = RgbColor(235, 66, 66).on_default();
    const BLUE: Style = RgbColor(2, 149, 235).on_default();
    const GREEN: Style = RgbColor(57, 219, 57).on_default();
    const AMBER: Style = RgbColor(245, 181, 61).on_default();
}
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use crate::cli::Cli;
use clap::Parser;
use tokio::{signal, sync::{mpsc, Mutex}};
enum Shutdown {
    Graceful,
    Borked,
}
fn main() -> anyhow::Result<()> {
    let body = async {
        let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_c = Arc::clone(&shutdown);
        let cmd = Arc::new(Mutex::new(Cli::parse()));
        let cmd_c = Arc::clone(&cmd);
        tokio::spawn(async move {
            match cmd_c.lock().await.run().await {
                Ok(_) => shutdown_tx.send(Shutdown::Graceful).unwrap(),
                Err(err) => {
                    {
                        let red = ::anstyle::RgbColor(235, 66, 66).on_default();
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "{1}{0}{1:#}\n",
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!("[ERROR]: {0}", err),
                                        );
                                        res
                                    },
                                    red,
                                ),
                            );
                        }
                    };
                    shutdown_tx.send(Shutdown::Borked).unwrap()
                }
            }
        });
        {
            #[doc(hidden)]
            mod __tokio_select_util {
                pub(super) enum Out<_0, _1> {
                    _0(_0),
                    _1(_1),
                    Disabled,
                }
                pub(super) type Mask = u8;
            }
            use ::tokio::macros::support::Future;
            use ::tokio::macros::support::Pin;
            use ::tokio::macros::support::Poll::{Ready, Pending};
            const BRANCHES: u32 = 2;
            let mut disabled: __tokio_select_util::Mask = Default::default();
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 0;
                disabled |= mask;
            }
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 1;
                disabled |= mask;
            }
            let mut output = {
                let mut futures = (signal::ctrl_c(), shutdown_rx.recv());
                let mut futures = &mut futures;
                ::tokio::macros::support::poll_fn(|cx| {
                        let mut is_pending = false;
                        let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                        for i in 0..BRANCHES {
                            let branch;
                            #[allow(clippy::modulo_one)]
                            {
                                branch = (start + i) % BRANCHES;
                            }
                            match branch {
                                #[allow(unreachable_code)]
                                0 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        _ => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_0(out));
                                }
                                #[allow(unreachable_code)]
                                1 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (_, fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        shutdown => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_1(out));
                                }
                                _ => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "internal error: entered unreachable code: {0}",
                                            format_args!(
                                                "reaching this means there probably is an off by one bug",
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                        if is_pending {
                            Pending
                        } else {
                            Ready(__tokio_select_util::Out::Disabled)
                        }
                    })
                    .await
            };
            match output {
                __tokio_select_util::Out::_0(_) => {
                    {
                        {
                            ::std::io::_print(
                                format_args!(
                                    "{0}\n",
                                    ::ansi_term::Colour::RGB(245, 181, 61)
                                        .paint(
                                            &{
                                                let res = ::alloc::fmt::format(
                                                    format_args!("\nMISSION ABORTED!!!!!!!!! (╯°□°）╯"),
                                                );
                                                res
                                            },
                                        ),
                                ),
                            );
                        }
                    };
                    shutdown_c.store(true, Ordering::Relaxed);
                }
                __tokio_select_util::Out::_1(shutdown) => {
                    match shutdown.unwrap_or(Shutdown::Borked) {
                        Shutdown::Graceful => shutdown_c.store(true, Ordering::Relaxed),
                        Shutdown::Borked => shutdown_c.store(false, Ordering::Relaxed),
                    }
                }
                __tokio_select_util::Out::Disabled => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "all branches are disabled and there is no else branch",
                        ),
                    );
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("failed to match bind"),
                        ),
                    );
                }
            }
        };
        match shutdown.load(Ordering::Relaxed) {
            true => {
                {
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0}\n",
                                ::ansi_term::Colour::RGB(57, 219, 57)
                                    .paint(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "Gracefully shutting down... \\(￣︶￣*\\))",
                                                ),
                                            );
                                            res
                                        },
                                    ),
                            ),
                        );
                    }
                };
                std::mem::drop(cmd);
                std::process::exit(0);
            }
            false => {
                {
                    let red = ::anstyle::RgbColor(235, 66, 66).on_default();
                    {
                        ::std::io::_eprint(
                            format_args!(
                                "{1}{0}{1:#}\n",
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("I borked... (┬┬﹏┬┬)"),
                                    );
                                    res
                                },
                                red,
                            ),
                        );
                    }
                };
                std::mem::drop(cmd);
                std::process::exit(1);
            }
        }
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
