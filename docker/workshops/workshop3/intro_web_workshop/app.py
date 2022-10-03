from flask import Flask, render_template, request, make_response 
import os
app = Flask(__name__)

@app.route("/",methods = ['GET', 'POST'])
def chall1():
    if(request.method!="POST"):
        return render_template("index.html",message="I prefer a different method of communication")
    if(request.headers["User_Agent"]!="UMASS BROWSER"):
        return render_template("index.html",message=f"I dont't like {request.headers['User-Agent']}, can you maybe change to UMASS BROWSER")
    if("fr" not in request.headers["Accept-Language"]):
        return render_template("index.html",message=f"I would appreciate it if you would accept the French language. Also remember rm -fr /")
    return render_template("index.html",message="UMASS{FRENCH_UMASS_BROWSER_3NTHUSI4ST}")
@app.route("/chall2")
def chall2():
    cookie = request.cookies.get('yummy_cookie')
    if(cookie is None):
        return render_template("chall2.html",message=f"I found Cookie:{request.cookies} but I am missing a Cookie called yummy_cookie.")
    if(cookie != "Chocolate_Chip"):
        return render_template("chall2.html",message=f"I found {request.cookies} but I only like Chocolate_Chip")
    return render_template("chall2.html",message="UMASS{CH0C0L4T3_CH1P_FTW}")

@app.route("/chall3",methods = ['GET', 'POST'])
def chall3():
    if(request.method=="GET"):
        return render_template("chall3.html",resp="No ping sent.")
    return os.popen(f'ping -c 3 {request.json["ip_to_ping"]}').read()
if __name__ == "__main__":
    app.run()