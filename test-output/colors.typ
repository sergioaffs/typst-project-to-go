#let parse_color_scheme(scheme_description) = {
	let color_scheme = (:)

	for scheme_entry in scheme_description {
		let color_spec = rgb("#000000")

		if scheme_entry.at("color_space") == "RGB" {
			color_spec = rgb(scheme_entry.color)
		}
		color_scheme.insert(scheme_entry.at("use"),color_spec)
	}

	color_scheme
}

#let get_color_scheme(scheme: "default") = {

	let color_scheme_info = yaml("color-schemes.yml")

	let selected_color_scheme = color_scheme_info.color_schemes.at(scheme)


	parse_color_scheme(selected_color_scheme)
}
