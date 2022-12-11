/// @description Insert description here
// You can write your code in this editor

var file_buffer = buffer_load("input");
var input = string_split(buffer_read(file_buffer, buffer_string), "\n");
buffer_delete(file_buffer);


var total = array_reduce(array_map(input, function(el, i) {
	if (el == "") return 0;
	var ranges = array_map(string_split(el, ","), function(el, i) { return string_split(el, "-")});
	if ((real(ranges[0][0]) >= real(ranges[1][0]) and real(ranges[0][1]) <= real(ranges[1][1])) or
		(real(ranges[1][0]) >= real(ranges[0][0]) and real(ranges[1][1]) <= real(ranges[0][1]))) {
		return 1;
	} else {
		return 0;
	}
}), function(prev, curr, i) { return prev + curr });

show_debug_message("part 1: " + string(total));

var total = array_reduce(array_map(input, function(el, i) {
	if (el == "") return 0;
	var ranges = array_map(string_split(el, ","), function(el, i) { return string_split(el, "-")});
	if ((real(ranges[0][0]) >= real(ranges[1][0]) and real(ranges[0][0]) <= real(ranges[1][1])) or
		(real(ranges[0][1]) >= real(ranges[1][0]) and real(ranges[0][1]) <= real(ranges[1][1])) or
		(real(ranges[1][0]) >= real(ranges[0][0]) and real(ranges[1][0]) <= real(ranges[0][1])) or
		(real(ranges[1][1]) >= real(ranges[0][0]) and real(ranges[1][1]) <= real(ranges[0][1]))) {
		return 1;
	} else {
		return 0;
	}
}), function(prev, curr, i) { return prev + curr });

show_debug_message("part 2: " + string(total));