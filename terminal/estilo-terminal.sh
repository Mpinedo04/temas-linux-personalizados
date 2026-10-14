# Estilos de COSMIC para la terminal: el prompt y una intro animada que cambian
# según el estilo puesto (~/.local/share/estilos/.actual).
#
# Se carga desde ~/.bashrc. Para quitar solo la intro: crea el archivo
# ~/.config/estilos/sin-intro (o pon ESTILO_INTRO=0 antes de esta línea).

[[ $- == *i* ]] || return 0

__estilo_dir="$HOME/.local/share/estilos"
__estilo_lib="$__estilo_dir/terminal"
__estilo_ps1_original=$PS1
__estilo_actual="--"

# Ruta al estilo de Windows: /home/ana/Documentos -> C:\Users\ana\Documentos
__estilo_winpath() {
    local p=$PWD
    if [[ $p == "$HOME"* ]]; then
        p="C:\\Users\\$USER${p#"$HOME"}"
    else
        p="C:$p"
    fi
    printf '%s' "${p//\//\\}"
}

__estilo_aplicar() {
    local s=""
    [[ -r $__estilo_dir/.actual ]] && read -r s < "$__estilo_dir/.actual"
    [[ $s == "$__estilo_actual" ]] && return
    __estilo_actual=$s
    case $s in
        hacker)
            PS1='\[\e[38;2;0;255;65m\]┌──(\[\e[1m\]\u@\h\[\e[22m\])-[\[\e[38;2;170;255;190m\]\w\[\e[38;2;0;255;65m\]]\n└─\[\e[1m\]\$\[\e[0m\] ' ;;
        cyberpunk)
            PS1='\[\e[38;2;255;43;214m\]▓▒░ \[\e[38;2;0;240;255m\]\u\[\e[38;2;252;238;10m\]@\h \[\e[38;2;255;43;214m\]░▒▓ \[\e[38;2;230;214;255m\]\w\n\[\e[38;2;0;240;255m\]❯\[\e[38;2;255;43;214m\]❯\[\e[0m\] ' ;;
        macos)
            PS1='\[\e[1m\]\u@\h\[\e[0m\] \[\e[38;2;0;122;255m\]\W\[\e[0m\] % ' ;;
        windows)
            PS1='\[\e[38;2;204;204;204m\]PS $(__estilo_winpath)>\[\e[0m\] ' ;;
        minimalista)
            PS1='\[\e[38;2;136;192;208m\]\W \[\e[38;2;129;161;193m\]❯\[\e[0m\] ' ;;
        *)
            PS1=$__estilo_ps1_original ;;
    esac
}

if [[ "$(declare -p PROMPT_COMMAND 2>/dev/null)" == "declare -a"* ]]; then
    PROMPT_COMMAND+=(__estilo_aplicar)
else
    PROMPT_COMMAND="__estilo_aplicar${PROMPT_COMMAND:+; $PROMPT_COMMAND}"
fi
__estilo_aplicar

# Intro solo en una terminal recién abierta (no en una shell dentro de otra).
__estilo_padre=""
read -r __estilo_padre < /proc/$PPID/comm 2>/dev/null
case $__estilo_padre in
    bash | sh | dash | zsh | fish | sudo | su | tmux* | screen | sshd* | code*) ;;
    *)
        if [[ ${ESTILO_INTRO:-1} != 0 && ! -e $HOME/.config/estilos/sin-intro && -t 0 && -t 1 ]] \
            && command -v python3 > /dev/null; then
            python3 "$__estilo_lib/intro.py" "$__estilo_actual"
        fi
        ;;
esac
unset __estilo_padre
