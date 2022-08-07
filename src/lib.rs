pub fn coucou() -> String {
	#[cfg(test)] {
		return String::from("coucou les tests");
	}
	#[cfg(not(test))] {
		return String::from("coucou la prod");
	}	
}

#[test]
fn test_coucou() {
	assert_eq!(coucou(), "coucou les tests");
}