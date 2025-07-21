#import "pckgs/CV/2025.1.0/CV-template.typ": *

#show: text.with(hyphenate: false)

#box(
	fill: gradient.linear(angle: 90deg, highlight_color_3.lighten(90%), highlight_color_3.lighten(85%)),
	outset: 1em,
	radius: 4pt,
	width: 90%, 
	[
		#lang(
		(
			"en": [= Publications],
			"de": [= Publikationen])
		)		
		#v(.9em)

		#custom-bullet(
			marker: [*2025*], 
				
					[#text(hyphenate: false, lang:"en")[*Threat Modeling: Charting Your Security Journey*] \ 
					#text(size: 0.8em)[Black Duck]\
					 	#link("https://www.brighttalk.com/webcast/13983/636246")[#text(size: 0.8em)[
								#lang(
									(
										"en": "Webinar (registration needed)",
										"de": "Webinar (Englisch, Anmeldung erforderlich)"
									)
								) 
							 #fa-external-link()]]]
					 
				// )
		)

		#custom-bullet(
			marker: [*2024*], 
				
					[#lang(
									(
										"en": [*About ransomware*],
										"de": [*Über Ransomware*]
									)
								) \ 
					#text(size: 0.8em)[TechRadar]\
					 	#link("https://www.techradar.com/pro/the-evolution-of-cybercrime-how-ransomware-became-the-weapon-of-choice")[#text(size: 0.8em)[
								#lang(
									(
										"en": "Article",
										"de": "Artikel (Englisch)"
									)
								) 
							 #fa-external-link()]]]
					 
				// )
		)

		#custom-bullet(
			marker: [*2023*], 
				
					[*Threat Modeling* \ 
					#text(size: 0.8em)[
						#lang((
							en: "Black Duck (coauthor)",
							de: "Black Duck (Co-Author)"
							))]\
					 	#link("https://www.blackduck.com/resources/white-papers/threat-modeling-best-practices.html")[#text(size: 0.8em)[
							#lang(
									(
										"en": "Whitepaper (registration needed)",
										"de": "Whitepaper (Englisch, Anmeldung erforderlich)"
									)
								) 
							 #fa-external-link()]]]
					 
				// )
		)

		#custom-bullet(
			marker: [*2013*], 
					[
						#lang(
									(
										"en": [*About biometrics*],
										"de": [*Über Biometrie*]
									)
								) \ 
						#text(size: 0.8em)[
							#lang(
									(
										"en": "ENTER.CO (Colombian magazine)",
										"de": "ENTER.CO (kolumbianische Zeitschrift)"
									)
							)						
						] \
					 	#link("https://www.enter.co/especiales/enterprise/136594-revision-v1/")[#text(size: 0.8em)[
							#lang(
									(
										"en": "Article  (Spanish)",
										"de": "Artikel  (Spanisch)"
									)
							)		
							#fa-external-link()]
					]	 
					]	
		)

		#custom-bullet(
			marker: [*2011*], 
					[
						#lang(
									(
										"en": [*About passwords*],
										"de": [*Über Passwörter*]
									)
								) \
					#text(size: 0.8em)[ENTER.CO] \
					 	#link("https://www.enter.co/empresas/seguridad/tener-una-contrasena-segura-la-clave-de-la-seguridad-en-linea/")[#text(size: 0.8em)[
							#lang(
									(
										"en": "Article  (Spanish)",
										"de": "Artikel  (Spanisch)"
									)
							)		
							#fa-external-link()]] ]
		)
		
		#custom-bullet(
			marker: [*2011*], [
					#lang(
									(
										"en": [*About web tools*],
										"de": [*Über Web Tools*]
									)
								) \ 
					#text(size: 0.8em)[ENTER.CO] \
					 	#link("https://www.enter.co/otros/tres-joyas-en-linea-que-mejoraran-su-experiencia-web/")[#text(size: 0.8em)[#lang((
							en: [Article (Spanish) #fa-external-link()],
							de: [Artikel (Spanisch) #fa-external-link()]
							))]] \
			]
		)
		
		#lang(
									(
										"en": [= Public mentions],
										"de": [= In der Presse]
									)
								)

		#custom-bullet(
			marker: [*2025*], 
					[#lang((
						en: [*About prime numbers in cryptography*],
						de: [*Über Primzahlen in Kryptographie*]
						)) \ 
					#text(size: 0.8em)[Teri Robinson, Security Boulevard] \
					 	#link("https://securityboulevard.com/2025/05/do-the-math-prime-number-breakthrough-could-upend-encryption/")[#text(size: 0.8em)[
							#lang(
									(
										"en": "Article",
										"de": "Artikel (Englisch)"
									)
								)
							 #fa-external-link()]] \ 
					 ]
		)

		#custom-bullet(
			marker: [*2010*], 
					[#lang((
						en: [*Innovation award*],
						de: [*Innovationspreis*]
						)) \ 
					#text(size: 0.8em)[Universidad de los Andes] \
					 	#link("https://www.semana.com/administracion/articulo/convertir-idea-empresa/102990/")[#text(size: 0.8em)[
							#lang(
									(
										"en": "Interview (Spanish)",
										"de": "Interview (Spanisch)"
									)
								)
							 #fa-external-link()]] \ 
					 ]
		)

		#custom-bullet(
			marker: [*2007*], 
					[*Interview* \ 
					#text(size: 0.8em)[
						#lang(
									(
										"en": [Mauricio Jaramillo Marín, \ _El Tiempo_ (Colombian newspaper)],
										"de": [Mauricio Jaramillo Marín, \ _El Tiempo_ (kolumbianische Zeitung)]
									)
								)
							] \
					 	#link("https://www.eltiempo.com/archivo/documento/MAM-2420476")[#text(size: 0.8em)[
							#lang(
									(
										"en": "Link (Spanish)",
										"de": "Link (Spanisch)"
									)
								)
							#fa-external-link()]] 

							#v(1em)
					 ]
					 
		)]
)
