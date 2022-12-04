/// @description Insert description here
// You can write your code in this editor

var file = file_text_open_read("input")
while (!file_text_eof(file))
{
    show_debug_message(file_text_readln(file));
}
file_text_close(file);