#import "@preview/fontawesome:0.5.0": *
// #import "@preview/timeliney:0.0.1"
#import "colors.typ"

#let color_scheme = colors.get_color_scheme(scheme: "new")
#let highlight_color_1 = color_scheme.at("primary")
#let highlight_color_2 = color_scheme.at("secondary")
#let highlight_color_3 = color_scheme.at("accent")
#let subdue_color = highlight_color_2.lighten(30%)


#let lang(property) = {
	context {
		let language = text.lang
		if {type(property)==dictionary and language in property} {property.at(language)} else {property}
	}
}

#let eval-lang(property) = {
	context {
		let language = text.lang
		let raw-string = if {type(property)==dictionary and language in property} {property.at(language)} else {property}
		eval(raw-string, mode: "markup", scope: (subdue_color: subdue_color))
	}
}

#let CV(
	name: "Undefined",
	title: "Undefined",
	location: "Undefined",
	email: "Undefined",
	language: "en",
	doc
) = {
	show "oem": smallcaps
	show "Archiv": text.with(hyphenate: false)
	
	set page(
		paper: "a4",
		numbering: "1 / 1",
		margin: (
				top: .5cm,
				left: 1.6cm,
				right: 2cm,
				bottom: 1.4cm
			)
	)

	set text(
		lang: language,
		size: 11pt,
		hyphenate: true,
		// font: ("Lato")
		font: ("Futura","Fira Sans")
	)
	
	show heading: text.with(hyphenate: false)

	show heading.where(level: 1): the_heading => text(size: 0.9em, highlight_color_1, {
		v(0.3em)
		smallcaps(the_heading)
		v(0.3em)
		// v(0.3em)
	})

	show heading.where(level: 2): the_heading => text(size: 0.9em, highlight_color_1, {
		v(0.3em)
		smallcaps(the_heading)
		v(0.2em)
		// v(0.3em)
	})

	show list: text.with(size: 0.8em)

	show link: set text(highlight_color_3)

	v(3em)

	align(center,
		[#text(size: 1.5em, weight: 600,highlight_color_1, smallcaps(name)) \ 
		#text(size: 1.2em, highlight_color_1, [#smallcaps(strong(title))]) \ 
		#text(size: 1.1em, highlight_color_1, [#fa-location-dot() #location #h(1em) #fa-envelope(solid: true) #link("mailto:"+email)])
		]
	)

	v(1.4em)

	doc
}

#let CV-job(
	title: none,
	company: none,
	period: none,
	description: none
) = [
	== #lang(title) \
	#text(size: 0.8em)[#fa-building() #smallcaps[#lang(company)] \ 
	#fa-calendar() #lang(period)] \
	// #repr(lang(description))
	
	#eval-lang(description)
]

#let CV-education(
	program: none,
	institution: none,
	title: none,
	period: none,
	description: none,
	thesis: none
) = [
	== #lang(program)#h(1fr)	#box(
			width: auto,
			stroke: 0.5pt+highlight_color_1,
			outset: 3pt,
			radius: 2pt,
			fill: highlight_color_1.lighten(90%)
		)[#fa-graduation-cap() #strong(delta: -300, lang(title))] \
	#fa-university() #smallcaps[#lang(institution)] \
	#fa-calendar() #lang(period) \
	#eval-lang(description) \
	#if thesis != none {
		context{
			if text.lang=="en" [*Thesis*: ] else [*Masterarbeit*: ];
			lang(thesis.topic);
			h(1fr);
			link(thesis.link)[
				#if text.lang=="en" [Archive] else [Archiv]
				#fa-external-link()
			]; 
		}
	}

]

/*
	Use star icons to represent ratings. Currently only integer values supported.
*/
#let rating(max: 3, value) = {
	set text(size: 0.8em)
	for i in range(0, value) [#fa-star(solid: true)]
	for i in range(value,max) [#fa-star()]
}

#let load_job_experiences(job_entries) = {
	let job_root = job_entries.jobs

	for job in job_root {
		CV-job(
			title: job.title,
			company: job.company,
			period: job.period,
			description: job.description,
		)
	}
}

#let load_education_experiences(education_entries) = {
	let education_root = education_entries.education

	for education in education_root {
		let thesis_details = none
		if "thesis" in education.keys() {thesis_details = education.thesis}

		CV-education(
			program: education.program,
			institution: education.institution,
			title: education.title,
			period: education.period,
			description: education.description,
			thesis: thesis_details
		)
	}
}

#let format_skill_entry(details) = {
	let formatted_details = ()

	for skill_entry in details {
		let formatted_entry = {
			if "level_experience" in skill_entry.keys() [
				#skill_entry.skill (#smallcaps[#skill_entry.level_experience |  #rating(skill_entry.rating))]
			] else [#skill_entry.skill  #rating(skill_entry.rating)] 
		}

		formatted_details.push(formatted_entry)
	}

	formatted_details.join([ #sym.diamond.small ])
}

#let load_skills(skill_entries) = {
	let skill_root = skill_entries.skills
	let skill_list = ()

	for (skill_category, details) in skill_root {
		skill_list.push(text(weight: "bold", highlight_color_1, skill_category))

		let skill_details = format_skill_entry(details)
		skill_list.push(skill_details)
	}

	grid(columns: (auto, auto), gutter: 1em, ..skill_list)
}

#let get_timeline_headers(timeline_data) = {
	let raw_headers = range(timeline_data.start+1, timeline_data.end)
	let parsed_headers = raw_headers.map(n => "'"+str(n).slice(2,4) )
	parsed_headers.insert(0, str(timeline_data.start))
	parsed_headers.push(str(timeline_data.end))

	parsed_headers
}

#let load_timeline(filename) = {
	let timeline_data = yaml(filename)
	let timeline_headers = get_timeline_headers(timeline_data)
	/*timeline_data.start*/

	text(size: 3em, red, [Timeline package does not support more than one label per line. Do not use yet!])

	timeliney.timeline(
	  show-grid: false,
	  {
	    import timeliney: *
	      //  group(..range(4).map(n => strong("Q" + str(n + 1))))
	    headerline(
	    	..timeline_headers
	    		.map(n=>group((n,12))))
	  

      task("Research the market", (0, 2), style: (stroke: 2pt + gray))
      task("Conduct user surveys", (1, 3), style: (stroke: 2pt + gray))
	    
	  }
	)
}

/*
	Create a bullet point with a custom bullet (e.g. a given icon or a text)
*/
#let custom-bullet(
	marker: "-", 
	scale: 1,
	content) = {
		grid(
			columns: (auto, auto),
			gutter: 0.7em,
			h(1em)+box(width: 2em, height: 1em,
				text(size: scale * 1em,
				align(center, marker))),
				content
		)
	}

#let comment(content) = {text(fill: subdue_color, size: 0.8em)[_#content _]}