#import "pckgs/CV/2025.1.0/CV-template.typ": *

#show: text.with(hyphenate: false)

		= #lang((
			en: "Skills",
			de: "Fähigkeiten"
			))
		#v(-0.6em)
		#text(size:0.8em)[
			#lang(
				(
					// en: [_Underlined=core expertise_],
					en: {comment("Underlined=core expertise")},
					de: {comment("(Schwerpunkte unterstrichen)")},
				)
				)]
    #v(0.6em)
    // #set text(hyphenate: false)
    #grid(columns: 2,
    column-gutter: 1em,
    row-gutter: 1.6em,
    align(center, text(size: 1.8em, fa-key())), 
    [ #lang(
				(
					en: [*IT-Security*],
					de: [*IT-Sicherheit*],
				)
			) \ 
    #text(size: 0.8em)[
			#lang(
				(
					en: [#underline[Threat modeling], #underline[cryptography],
    application security, compliance (e.g. ISO 27001, ISO/SAE 21434), online voting],
					de: [#underline[Threat Modeling], #underline[Kryptographie], Anwendungsicherheit (Eng: AppSec), Compliance (z.B. ISO 27001, ISO/SAE 21434), E-voting],
				)
				)
			 ]],
     text(size: 1.8em, fa-laptop-code()),
     [ #lang(
				(
					en: [*Programming languages*],
					de: [*Programmiersprachen*],
				)
			) \
     #text(size: 0.8em)[#underline[Python], #underline[Java], Rust, C]
     ], 
     text(size: 1.8em, fa-chalkboard-teacher()),
     [#lang(
				(
					en: [*Knowledge sharing*],
					de: [*Kommunikation und Training*],
				)
			)\
     #text(size: 0.8em)[#lang(
				(
					en: [
						#underline[Technical writing],
						#underline[typesetting], 
						#underline[training] (production, hosting),
						template maintenance
					],
					de: [
						#underline[Technical Writing],
						#underline[Typographie],
						#underline[Training] (Entwicklung, Durchführung),
						Templateentwicklung],
				)
			)]],
    align(center,text(size: 1.8em, fa-circle-dot())),
     [*Misc*\
     #text(size: 0.8em)[
			#lang((
				"en": "UX design, definition and security  of business processes, iconography",
				"de":	"UX, Analyse von Geschäftsprozessen mit Fokus auf Sicherheit, visuelle Kommunikation")
			)]]
    )


		// #custom-bullet(
		// 	marker: fa-bolt(), 
		// 	[Risk analysis #h(1fr) #text(size: 0.8em, smallcaps[Company expert])]
		// )
		// #custom-bullet(
		// 	marker: fa-key(), 
		// 	[Cryptography #h(1fr) #text(size: 0.8em, smallcaps[M. Sc.])]
		// )
		// #custom-bullet(
		// 	marker: fa-file-text(), 
		// 	[Template automation #h(1fr) #text(size: 0.8em, smallcaps[Company expert])]
		// )
		// #custom-bullet(
		// 	marker: fa-laptop-code(), 
		// 	[Tech. writing #h(1fr) #text(size: 0.8em, smallcaps[10+ years])]
		// )
		// #custom-bullet(
		// 	marker: fa-java(), 
		// 	[Java #h(1fr) #text(size: 0.8em, smallcaps[10+ years, trainer])]
		// )
		// #custom-bullet(
		// 	marker: fa-python(),
		// 	[Python #h(1fr) #text(size: 0.8em, smallcaps[10+ years])]
		// )
