from django.shortcuts import render, get_object_or_404
from django.http import HttpResponse, HttpResponseRedirect, Http404
from django.urls import reverse
from django.db.models import F
from django.core.serializers import serialize
from api.views import internalgetserverstats
from django.utils import timezone
from datetime import datetime

import json, urllib, pytz, math

def index(request):
	context = {
		'page_title': 'Davis Ellwood | Home',
		'body_template': 'home/Home.html',
		'age': 25,
	}
	return render(request, 'header/PageLayout.html', context)

def work(request):
	context = {
		'page_title': 'Davis Ellwood | Work',
		'body_template': 'work/MainWorkPage.html',
	}
	return render(request, 'header/PageLayout.html', context)

def project(request, project):
	title = 'Davis Ellwood | '
	technologies = None
	features = None
	main_heading = None
	main_description = None
	footer = None
	stats_json = None
	if(project == 'skaterxl'):
		title += 'SkaterXL Mods'

		main_heading = 'SkaterXL Multiplayer Mod'
		main_description = '''The skateboarding simulator SkaterXL launched as a single-player only game. Wanting to improve my own experience as well as the experience of others I started to work on an extension to add multiplayer. As of today, the mod attracts over 300 daily downloads and up to 40 concurrent players on just the 4 dedicated public servers. During the process of adding multiplayer, I've come across many unique problems that required solving. One of these problems was reducing bandwidth by nearly 70% using a unique combination of native/custom lossy and lossless compression techniques. How I approached this problem and others, as well as the technologies used on the project, can be found below.'''
		
		technologies = ['C#', 'C# Reflection', 'Newtonsoft JSON', 'UDP/TCP Sockets', 'C# Harmony Lib', 'IL Code', 'Unity Game Engine', 'Python/Django', 'PostgreSQL', 'git']
		
		features = [
			{
				'name':'singleton',
				'heading':'Singleton Object Design',
				'description':'''Upon looking at the code of the game it's clear that it was made without multiplayer in mind. All the controller classes for the player/skateboard are singletons. My initial plan was to spawn in other players and send each person's input data over the network to recreate their movement. However, because of the singleton design, any attempts to spawn a duplicate player resulted in a fatal error and crashing of the program.\n\nThe solution I came up with was to disable all the controller scripts before duplication and send bone transform data instead of input data. This avoided any crashes as the singleton check wouldn't run on the duplicate objects as they're disabled and also got around using the input controller by just sending the player state each frame.'''
			},
			{
				'name':'bandwidth',
				'heading':'High Bandwidth',
				'description':'''Sending full transform data as a solution to the singleton design allowed for accurate player states across the network, although it came at a significant bandwidth cost. A player in SkaterXL has 72 bones with 3 floats representing position and 4 floats for rotation, at 4 bytes per float it totals 2016 bytes per update, so in a 10 person lobby at 30 updates per second that's 590KB/s. A download rate of 590KB/s just for player animation was not acceptable, but as I can't duplicate scripts I have no way to simulate the transforms locally.\n\nThe solution I came up with was multi-layered, first I implemented standard compression on animation data sent over the network. The next steps were to send keyframes using half-precision floats which only use 2 bytes, and send non-keyframes as deltas from the previous frame at full precision as not all bones will move/rotate the zeros compress away nicely. Lastly, I convert the rotation from a quaternion to Euler angle form eliminating a float per bone. The combination of these techniques provides very accurate animation data while reducing the average bytes per update to less than 33% of uncompressed animation at approximately 687 bytes per update. '''
			},
			{
				'name':'debugging',
				'heading':'Insufficient Debugging Tools',
				'description':'''Generally, debugging is rather pain-free, modern debuggers come with many powerful tools to view memory and CPU profiles, watch variables, and set breakpoints. Although when modifying release code that I don't own, this process becomes rather difficult as I don't have access to any debugging tools.\n\nOne example of this is when there was a memory-leak when players disconnected from the server. The issue was caused by texture memory not being garbage collected when the remote client's object was destroyed. Using a debugger I could have viewed memory or watched variables to see that the textures weren't being garbage collected on destruction.\n\nI eventually solved the problem by combing through code and keeping track of every object that was created on spawn until it's deletion and manually unloading the textures. However, with a proper debugger, this multi-hour process could have been done in 10 minutes with a memory snapshot before and after the player disconnects. Over time I've become more able to predict problems before they occur, as this debugging process required me to more deeply understand my code. '''
			},
			{
				'name':'unet',
				'heading':'Unity UNet',
				'description':'''In the first iterations of the Multiplayer Mod, I was using Unity's built-in solution for handling networking, UNet, with the Low-level API Transport Layer. However, requests for me to support custom clothing required me to drop UNet. UNet is designed without user-generated content in mind, so there's no way to send medium to large-sized files through UNet quickly and reliably.\n\nUpon realizing the lack of support for large reliable messages and wanting to find a solution that supported user-generated content, I switched to a custom solution using low-level TCP/UDP sockets. After a few iterations of the mod with a custom networking solution, however, I became aware of the open-source Game Networking Sockets by Valve. Generally, in programming, a problem you're facing has already been solved and there's no need to reinvent the wheel. In this situation, Game Networking Sockets was perfect for my needs as it had a connection system built entirely on UDP for quick transfers, transfer of large reliable messages, automatic packet splitting/joining, and a plethora of network debugging tools. After implementing Game Networking Sockets the transfer of texture files was only limited by the maximum configured sending rate and I could easily test how the game would play over real network conditions on a local server. '''
			}
		]

		stats_json = internalgetserverstats()
		footer = 'work/SkaterXLFooter.html'

	#REMOVE THIS SECTION
	# elif(project == 'website'):
	# 	title += 'Website'

	# 	main_heading = 'Davis Ellwood portfolio website and API'
	# 	main_description = '''In 2019 I set out to create a simple portfolio website to showcase interesting and unique projects I've created, ironically it has become one of my most complex endeavours and a staple of the portfolio it was meant to showcase. Shortly after setting up a barebones back-end for displaying webpages, my ambitions heightened to creating an API that could be accessed from other projects and now receives over 3000 requests per hour. Below you can find a list of technologies, features of my API, and development challenges associated with those features.'''
		
	# 	technologies = ['Python', 'Django', 'APScheduler', 'PostgreSQL', 'Heroku', 'Bootstrap', 'Javascript', 'JQuery', 'Chart.js', 'git']
		
	# 	features = [
	# 		{
	# 			'name':'server', 
	# 			'heading':'Live Server List', 
	# 			'description':"The first feature I added to my websites was the ability to receive a list of live servers. It works with API keys generated for verified server hosts whos server sends POST requests to the server API endpoint. All the server info is included with the POST and is then updated in the database if they have a valid API key. A background process then removes any servers that haven't been updated for 30 seconds. \n\n The second half of the API is for the client-side where they send a GET to the server list API endpoint. This returns a JSON object with all the servers and their information to be displayed."
	# 		},
	# 		{
	# 			'name':'username',
	# 			'heading':'Username Reservation',
	# 			'description':'''The second feature I wanted to tackle was reserving usernames for use in the multiplayer mod securely. The game and gameplay server already use a private/public key pair for secure communication so I didn't need to worry about a man in the middle attack there. However, the problem is that I also need to prevent malicious server operators from stealing someone elses credentials since I don't operate the gameplay servers. \n\n The solution I decided upon was a 3 step approach. First the player asks my webserver for an encryption key to use on their GUID to secure it from a potentially malicious server operator. After being sent with end-to-end encryption to the gameplay server the first layer of encryption on the players GUID is removed. The gameplay server then sends the encrypted GUID along with the connecting players IP to my webserver where it's decrypted and sends back the appropriate username as a response. This approach ensures that no matter where along the chain an attacker has access they only see an encrypted string or the publicly available username and can never impersonate another player.'''
	# 		}
	# 	]
	elif(project == 'blades'):
		title += 'Blades of Orterra'

		main_heading = 'Blades of Orterra Multiplayer Arena Combat Game'
		main_description = '''Blades of Orterra is possibly my most complex project to date being a fully featured videogame released in early 2018. During development I worked as the lead gameplay and network programmer for a multidisciplinary team including another gameplay programmer, two 3D modelers, an animator, a particle effects designer, UI designer, sound designer, and community manager. As most of my projects before this were solo or two person development teams this project taught me a lot about how to effectively integrate and communicate with technical and non-technical team members in a professional environment.'''

		technologies = ['C++', 'Unreal Engine 4', 'Perforce', 'Slack', 'Trello', 'HacknPlan']

		features = [
			{
				'name':'combat',
				'heading':'Real Time Sword Combat'
			},
			{
				'name':'anticheat',
				'heading':"Anti-Cheat"
			}
		]
	else:
		raise Http404("Tried to access page /work/" + project)

	context = {
		'page_title': title,
		'body_template': 'work/IndividualWorkTemplate.html',
		'stats_json': stats_json,
		'main_heading': main_heading,
		'main_description': main_description,
		'technologies': technologies,
		'features': features,
		'footer': footer
	}
	return render(request, 'header/PageLayout.html', context)

def handle404(request, exception):
	context = {
		'page_title': 'Davis Ellwood | Page Not Found',
		'body_template': 'Custom404.html',
	}
	return render(request, 'header/PageLayout.html', context)