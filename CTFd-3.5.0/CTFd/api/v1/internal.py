from typing import List

from flask import request
from flask_restx import Namespace, Resource
from flask import abort, render_template, request, url_for

from CTFd.api.v1.helpers.request import validate_args
from CTFd.api.v1.helpers.schemas import sqlalchemy_to_pydantic
from CTFd.api.v1.schemas import APIDetailedSuccessResponse, APIListSuccessResponse
from CTFd.models import Awards, Users, db
from CTFd.schemas.awards import AwardSchema
from CTFd.utils.config import is_teams_mode
from CTFd.utils.decorators import require_verified_emails
from CTFd.utils import config, get_config

from CTFd.utils.user import (
    authed,
    get_current_team,
    get_current_team_attrs,
    get_current_user,
    get_current_user_attrs,
    is_admin,
)
from CTFd.utils.helpers.models import build_model_filters
import requests
internal_namespace = Namespace("internal", description="Endpoint to talk to our internal RUST server")
@internal_namespace.route("")
class InternalResponse(Resource):
    @require_verified_emails
    def get(self):
        if authed():
            user = get_current_user()
            team = get_current_team()
            if is_admin():
                pass
            else:
                if config.is_teams_mode() and team is None:
                    abort(403)
        else:
            return {"Error":"You must be logged in to view this stuff :)"},403
        r1 = requests.get('http://127.0.0.1:8080/api/boxes')
        return {"success": True, "Data": r1.json()}, 200






