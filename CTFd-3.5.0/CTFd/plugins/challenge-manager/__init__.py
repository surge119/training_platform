from crypt import methods
from nis import cat
from random import seed
from traceback import print_tb
from flask import abort, request, render_template, Blueprint
from flask_sqlalchemy import SQLAlchemy
from CTFd.models import db, Challenges
from CTFd.utils import config
from CTFd.utils.decorators import admins_only
from CTFd.plugins import bypass_csrf_protection, register_plugin_assets_directory
from CTFd.plugins.dynamic_challenges import DynamicChallenge


challenge_manager = Blueprint(
    'challenge_manager', __name__, template_folder="templates")


SELECTED_CHALLENGES = "selected_challenges"
CHALLENGE_CATEGORY = "challenge_category"
CHALLNEGE_TYPE = "challenge_type"
CHALLENGE_VALUE = "challenge_value"


def update_challenges(data):
    """"""
    challenges = data["selected_challenges"]
    for challenge in challenges:
        chall = Challenges.query.get(challenge)
        update_challenge_category(chall, data[CHALLENGE_CATEGORY])
        update_challenge_value(chall, data[CHALLENGE_VALUE])
        update_challenge_type(chall, data)

        if chall.type == "dynamic":
            update_dynamic_values(chall, data["dynamic_data"])


def update_challenge_category(challenge, category):
    """"""
    if category is not None and category is not "":
        update_challenge(challenge, "category", category)


def update_challenge_value(challenge, value):
    """"""
    if value is not None and value is not "":
        update_challenge(challenge, "value", value)


def update_challenge_type(chall, data):
    """"""
    chall_id = chall.id

    if data[CHALLNEGE_TYPE] == "dynamic":
        dyn_chall = DynamicChallenge.query.get(chall_id)
        if dyn_chall is None:
            create_dynamic_chall(chall, data["dynamic_data"])
    elif (data[CHALLNEGE_TYPE] == "standard"):
        update_challenge(chall, "type", "standard")

    # update_challenge(chall, "type", "dynamic")


def update_dynamic_values(chall, data):
    """"""


def create_dynamic_chall(chall, data):
    """"""
    update_challenge(chall, "type", "dynamic")
    db.session.commit()

    dyn_chall = DynamicChallenge(
        id=chall.id, initial=data["initial"], minimum=data["minimum"], decay=["decay"])
    print(dyn_chall)

    db.session.commit()

    db.session.add(dyn_chall)
    # db.session.commit()
    # add_challenge(dyn_chall)


def update_challenge(challenge, data_str, data_val):
    """"""
    setattr(challenge, data_str, data_val)
    db.session.commit()


def add_challenge(challenge):
    """"""
    db.session.add(challenge)
    db.session.commit()


def validate_challenges(challenges):
    """"""
    selected_challenges = list()
    for challenge in challenges:
        try:
            selected_challenges.append(int(challenge))
        except:
            print(f"Invalid challenge ID provided: {challenge}")

    return selected_challenges


@challenge_manager.route("/admin/plugins/challenge_manager", methods=['GET', 'POST'])
@admins_only
@bypass_csrf_protection
def index():
    """"""
    if request.method == 'GET':
        challenges = get_challenges()
        return render_template("manager.html", challenges=challenges)
    elif request.method == 'POST':
        print(DynamicChallenge.query.all())
        print(Challenges.query.all())
        data = request.json
        data[SELECTED_CHALLENGES] = validate_challenges(
            data[SELECTED_CHALLENGES])
        update_challenges(data)
        return {"success": True, "challenges": get_challenges()}, 200


def get_challenges():
    """"""
    challenges = dict()

    for challenge in Challenges.query.all():
        chall = {
            "id": challenge.id,
            "name": challenge.name,
            "value": challenge.value,
            "type": challenge.type,
            "category": challenge.category,
            "description": challenge.description,
        }
        challenges[challenge.id] = chall
    return challenges


def load(app):
    app.register_blueprint(challenge_manager)
    register_plugin_assets_directory(
        app, base_path="/plugins/challenge-manager/assets/")
