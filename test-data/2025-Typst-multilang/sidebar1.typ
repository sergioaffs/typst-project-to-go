#import "@local/CV:2025.1.0": *

#box(
	fill: gradient.linear(angle: 90deg, highlight_color_3.lighten(90%), highlight_color_3.lighten(85%)),
	outset: 1em,
	radius: 4pt,
	width: 90%, 
	[

		#lang(
			(
				"en": [= Languages],
				"de": [= Sprachen]
			)
		)
		#v(.9em)
		#grid(columns: (2.4em, 1fr),
    column-gutter: 1em,
    row-gutter: 1.6em,
		// SPANISH
		image(width: 2.6em, height: 2em, "img/flag-es.png"), 
			[#lang(
			(
				"en": [*Spanish*],
				"de": [*Spanisch*]
			)
		) \
	 #text(size: 0.8em, smallcaps[
		#lang(
			(
				"en": "Native",
				"de": "Muttersprache"
			)
		)
	 ])],
	// ENGLISH
	image(width: 2.6em, height: 2em, "img/flag-us-uk.png"),
	[#lang(
			(
				"en": [*English*],
				"de": [*Englisch*]
			)
		) \
	 #text(size: 0.8em, smallcaps[
		#lang(
			(
				"en": "Fluent",
				"de": "Verhandlungssicher"
			)
		)
	 ])], 
	 // GERMAN
	image(width: 2.6em,height: 2em, "img/flag-de.png"), 
	[#lang(
			(
				"en": [*German*],
				"de": [*Deutsch*]
			)
		) \
	 #text(size: 0.8em, smallcaps[
		#lang(
			(
				"en": "Work proficiency",
				"de": "Fließend"
			)
		)
	 ])],
			// [*Deutsch*\ #text(size: 0.8em, smallcaps[Fließend])]
		)
	#include("skills.typ")


	#lang(
			(
				"en": [= Online presence],
				"de": [= Online]
			)
		) 
	#v(0.6em)
	#grid(columns: (2.4em, 1fr),
    column-gutter: 1em,
    row-gutter: 1.6em,
		align(center, text(size: 2em, fa-linkedin())),
		[*LinkedIn* \ 
		#link("https://www.linkedin.com/in/sergioaffs/en")[#text(size: 0.8em)[
				#lang(
				(
					"en": "Profile",
					"de": "Profil (Englisch)"
				)
			) 
			#fa-external-link()]]],
		align(center, text(size: 2em, fa-stack-exchange())),
	[*StackExchange* \ 
	#link("https://crypto.stackexchange.com/users/25371/sergio-a-figueroa")[
		#text(size: 0.8em)[
				#lang(
			(
				"en": "Cryptography",
				"de": "Cryptography (Englisch)"
			)
		) 
			#fa-external-link()]] \ 
					 	// #link("https://security.stackexchange.com/users/104208/sergio-a-figueroa")[#text(size: 0.8em)[IT Security (Englisch) #fa-external-link()]]
						]
	)

	#v(1em)

		
	]

	
)
