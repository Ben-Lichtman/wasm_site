import http.server

PORT = 80

class MyHandler(http.server.SimpleHTTPRequestHandler):
	def do_GET(self):
		print(self.directory)
		print(self.path)
		path = self.path
		if not (path.startswith("/pkg") or path == "/" or path.endswith(".css") or path.endswith(".js") or path.startswith("/static")):
			self.path = "/index.html"

		http.server.SimpleHTTPRequestHandler.do_GET(self)



with http.server.HTTPServer(("", PORT), MyHandler) as httpd:
    print("serving at port", PORT)
    httpd.serve_forever()
