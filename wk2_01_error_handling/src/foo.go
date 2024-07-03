my_function() -> (ValuePtr, i32) {
	return (ValuePtr::new(0), 0);
}

main() {
	let (value, ok) = my_function();
	if ok == 0 {
		println!("ok");
	}

	value
}
