use std::fmt;

const TOP_LEVEL_COMMANDS: &[&str] = &[
    "help",
    "version",
    "status",
    "sandbox",
    "dump-manifests",
    "bootstrap-plan",
    "agents",
    "mcp",
    "skills",
    "system-prompt",
    "login",
    "logout",
    "init",
    "prompt",
    "completions",
];

const TOP_LEVEL_FLAGS: &[&str] = &[
    "--help",
    "-h",
    "--version",
    "-V",
    "--model",
    "--output-format",
    "--permission-mode",
    "--dangerously-skip-permissions",
    "--allowedTools",
    "--allowed-tools",
    "--resume",
    "--print",
    "-p",
];

const COMPLETION_SHELLS: &[&str] = &["bash", "zsh", "powershell"];
const OUTPUT_FORMATS: &[&str] = &["text", "json"];
const PERMISSION_MODES: &[&str] = &["read-only", "workspace-write", "danger-full-access"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionShell {
    Bash,
    Zsh,
    PowerShell,
}

impl CompletionShell {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "bash" => Some(Self::Bash),
            "zsh" => Some(Self::Zsh),
            "powershell" => Some(Self::PowerShell),
            _ => None,
        }
    }

    pub fn supported_shells() -> &'static [&'static str] {
        COMPLETION_SHELLS
    }

    pub fn render(self) -> String {
        match self {
            Self::Bash => render_bash(),
            Self::Zsh => render_zsh(),
            Self::PowerShell => render_powershell(),
        }
    }
}

impl fmt::Display for CompletionShell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::PowerShell => "powershell",
        };
        f.write_str(name)
    }
}

fn render_bash() -> String {
    let commands = TOP_LEVEL_COMMANDS.join(" ");
    let flags = TOP_LEVEL_FLAGS.join(" ");
    let shells = COMPLETION_SHELLS.join(" ");
    let output_formats = OUTPUT_FORMATS.join(" ");
    let permission_modes = PERMISSION_MODES.join(" ");

    format!(
        r#"_claw() {{
    local cur prev first
    COMPREPLY=()
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    prev="${{COMP_WORDS[COMP_CWORD-1]}}"
    first="${{COMP_WORDS[1]}}"

    case "$prev" in
        --output-format)
            COMPREPLY=( $(compgen -W "{output_formats}" -- "$cur") )
            return 0
            ;;
        --permission-mode)
            COMPREPLY=( $(compgen -W "{permission_modes}" -- "$cur") )
            return 0
            ;;
        completions)
            COMPREPLY=( $(compgen -W "{shells}" -- "$cur") )
            return 0
            ;;
    esac

    if [[ $COMP_CWORD -eq 1 ]]; then
        COMPREPLY=( $(compgen -W "{commands} {flags}" -- "$cur") )
        return 0
    fi

    if [[ "$first" == "completions" && $COMP_CWORD -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "{shells}" -- "$cur") )
        return 0
    fi

    COMPREPLY=( $(compgen -W "{flags}" -- "$cur") )
}}

complete -F _claw claw
"#
    )
}

fn render_zsh() -> String {
    let commands = TOP_LEVEL_COMMANDS.join(" ");
    let shells = COMPLETION_SHELLS.join(" ");
    let output_formats = OUTPUT_FORMATS.join(" ");
    let permission_modes = PERMISSION_MODES.join(" ");

    format!(
        r#"#compdef claw

local -a commands
commands=({commands})

_arguments -C \
  '(-h --help)'{{-h,--help}}'[Show help]' \
  '(-V --version)'{{-V,--version}}'[Show version information]' \
  '--model=[Override the active model]:model:' \
  '--output-format=[Non-interactive output format]:format:({output_formats})' \
  '--permission-mode=[Permission mode]:mode:({permission_modes})' \
  '--dangerously-skip-permissions[Skip all permission checks]' \
  '--allowedTools=[Restrict enabled tools]' \
  '--allowed-tools=[Restrict enabled tools]' \
  '--resume=[Resume a saved session]' \
  '--print[Force text output]' \
  '-p[Prompt shorthand]' \
  '1:command:($commands)' \
  '*::arg:->args'

case $state in
  args)
    case $words[2] in
      completions)
        _values 'shell' {shells}
        ;;
    esac
    ;;
esac
"#
    )
}

fn render_powershell() -> String {
    let commands = TOP_LEVEL_COMMANDS
        .iter()
        .map(|command| format!("'{command}'"))
        .collect::<Vec<_>>()
        .join(", ");
    let flags = TOP_LEVEL_FLAGS
        .iter()
        .map(|flag| format!("'{flag}'"))
        .collect::<Vec<_>>()
        .join(", ");
    let shells = COMPLETION_SHELLS
        .iter()
        .map(|shell| format!("'{shell}'"))
        .collect::<Vec<_>>()
        .join(", ");
    let output_formats = OUTPUT_FORMATS
        .iter()
        .map(|value| format!("'{value}'"))
        .collect::<Vec<_>>()
        .join(", ");
    let permission_modes = PERMISSION_MODES
        .iter()
        .map(|value| format!("'{value}'"))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        r#"$commandNames = @({commands})
$topLevelFlags = @({flags})
$completionShells = @({shells})
$outputFormats = @({output_formats})
$permissionModes = @({permission_modes})

Register-ArgumentCompleter -Native -CommandName claw -ScriptBlock {{
    param($wordToComplete, $commandAst, $cursorPosition)

    $tokens = $commandAst.CommandElements | ForEach-Object {{ $_.Extent.Text }}
    $current = if ($null -ne $wordToComplete) {{ $wordToComplete }} else {{ '' }}
    $previous = if ($tokens.Count -ge 2) {{ $tokens[$tokens.Count - 2] }} else {{ '' }}
    $firstArg = if ($tokens.Count -ge 2) {{ $tokens[1] }} else {{ '' }}

    if ($previous -eq '--output-format') {{
        $candidates = $outputFormats
    }} elseif ($previous -eq '--permission-mode') {{
        $candidates = $permissionModes
    }} elseif ($previous -eq 'completions' -or ($firstArg -eq 'completions' -and $tokens.Count -le 3)) {{
        $candidates = $completionShells
    }} elseif ($tokens.Count -le 2) {{
        $candidates = $commandNames + $topLevelFlags
    }} else {{
        $candidates = $topLevelFlags
    }}

    foreach ($candidate in $candidates | Where-Object {{ $_ -like "$current*" }}) {{
        [System.Management.Automation.CompletionResult]::new($candidate, $candidate, 'ParameterValue', $candidate)
    }}
}}
"#
    )
}
