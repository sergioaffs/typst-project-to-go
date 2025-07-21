#import "pckgs/CV/2025.1.0/CV-template.typ": *

#let CV_language = {
	if "lang" in sys.inputs {
		let custom_language = sys.inputs.at("lang")

		assert(custom_language in ("en", "de"), message: "Language selected is not supported")

		custom_language
	} else {"en"}
}

#let CV_language = "de"

#show: doc => CV(
	name: "Sergio A. Figueroa",
	title: "Information Security Consultant",
	location: lang(
		(
			"en": "Augsburg, Germany",
			"de": "Augsburg, Deutschland"
		)
	),
	email: "sergioaffs@gmail.com",
	language: CV_language,
	doc
)

#let job-list = load_job_experiences(yaml("jobs.yml"))
#let education-list = load_education_experiences(yaml("education.yml"))

#v(2em)

#grid(
	columns: (35%, 60%),
	gutter: 2.6em,
	[#include("sidebar1.typ")],
	[
		#lang((
			"en": [= Work experience (11 years) ],
			"de": [= Berufserfahrung (11 Jahre)  ]
		)) // By end June 2025. (Counting each work experience: 11y5m=8y41m=2y1m+1y4m+4y7m+2y8m+9m)
		#job-list

	])

#v(1em)

#set page(margin: (top: 3cm))

#grid(
	columns: (35%, 60%),
	gutter: 2.6em,
	[#include("sidebar2.typ")],
	[
		#lang(
			(
				"en": [= Education],
				"de": [= Ausbildung]
			)
		)
		#education-list
	])

#v(1.6em)


