from flask import Flask, render_template,request,render_template_string
from werkzeug.debug import DebuggedApplication


app = Flask(__name__)
app.config['DEBUG'] = True

@app.route('/')
def index():
    name = request.args.get('name') or None
    template = '''
        <h1> Good Afternoon {}</h1>
    '''.format(name)
    print(template)
    return render_template_string(template),200
if __name__ == '__main__':
    app.run(host='0.0.0.0')