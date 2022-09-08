import requests
from flask import abort, request, render_template
from CTFd.utils import config
from CTFd.plugins import (
    bypass_csrf_protection)
from CTFd.utils.user import (
    authed,
    get_current_team,
    get_current_user,
    is_admin,
)

# Define IP address and port here
ADDR = "10.10.10.2:8000"

#simple flask app at this point. Make sure no potentially sensitive info is leaked
def load(app):
    app
    print("Rust Plugin loaded")
    @app.route('/api/UMASS/get_boxes', methods=['GET'])
    def get_boxes():
        if authed() is False:
            return {"Error":"You must be logged in to view this stuff :)"},403
        r1 = requests.get('http://' + ADDR + '/api/boxes')
        data = {
            "networks":[]
        }
        for key,val in r1.json().items():
            labs = []
            for name,lab in val['services'].items():
                print(lab)
                labs.append({
                    "container_name":name,
                    "description":lab["description"]
                })
            net_details = {
                "name":key,
                "labs":labs
            }
            data["networks"].append(net_details)
        return {"success": True, "Data":data}, 200

    @app.route("/api/UMASS/start_box",methods=['POST'])
    @bypass_csrf_protection
    def start_box():
        if is_admin() is not True:
            return {"Data":"Invalid Perms"},403
        if authed():
            user = get_current_user()
            team = get_current_team()
            if is_admin():
                pass
            else:
                if config.is_teams_mode() and team is None:
                    abort(403)
        else:
            return {"Error":"You must be logged in to start a box"},403
        r1 = requests.post('http://' + ADDR + '/api/start_box',headers={
            "content-type":"application/json"
        },json={
            "name":request.json['container_name']
        })
        return {"success": r1.json() is True, "Data": r1.text}, 200

    @app.route("/api/UMASS/stop_box",methods=['POST'])
    @bypass_csrf_protection
    def stop_box():
        if is_admin() is not True:
            return {"Data":"Invalid Perms"},403
        if authed():
            user = get_current_user()
            team = get_current_team()
            if is_admin():
                pass
            else:
                if config.is_teams_mode() and team is None:
                    abort(403)
        else:
            return {"Error":"You must be logged in to start a box"},403
        r1 = requests.post('http://' + ADDR + '/api/stop_box',headers={
            "content-type":"application/json"
        },json={
            "name":request.json['container_name']
        })
        return {"success": r1.json() is True, "Data": r1.text}, 200

