window.onload = () => {
    let challenges = document.getElementById("challenges").children;
    const challengesList = Array.from(challenges, chall => { return { ...chall.dataset } });

    addCheckboxListener(challengesList);
    addUpdateChallengesListener();
};

let selectChallenges = [];

/**
 * Add event listeners for checkbox
 * @param {Array} challenges 
 */
function addCheckboxListener(challenges) {
    let challengeIds = challenges.map(challenge => { return challenge['challenge'] });

    /**
     * Update the value of the checkbox
     * @param {string} id The id of the element to update
     * @param {boolean} status The value of the checkbox status
     */
    const setCheckBox = (id, status) => {
        document.getElementById(id).checked = status;
    };

    const addChall = id => { selectChallenges.push(id); selectChallenges.sort() };
    const remChall = id => { selectChallenges = selectChallenges.filter(e => e != id).sort(); };

    // Select all
    let selectAllCheckbox = document.getElementById("selectAll");
    selectAllCheckbox.addEventListener("change", e => {
        if (selectAllCheckbox.checked) {
            selectChallenges = challengeIds;
            challengeIds.forEach(id => setCheckBox(id, true));
        } else {
            selectChallenges = [];
            challengeIds.forEach(id => setCheckBox(id, false));
        }
    });

    // Individual select
    for (let chall of challenges) {
        let id = chall['challenge'];
        let checkbox = document.getElementById(id);
        checkbox.addEventListener("change", e => {
            if (checkbox.checked) {
                addChall(id);
            } else {
                remChall(id);
            }
        });
    }
}

function addUpdateChallengesListener() {
    document.getElementById("updateChallenges").addEventListener("click", e => {
        let challengeCategory = document.getElementById("challengeCategory").value;
        let challengeType = document.getElementById("challengeType").value;
        let challengeValue = document.getElementById("challengeValue").value;

        fetch("/admin/plugins/challenge_manager", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "selected_challenges": selectChallenges,
                "challenge_category": challengeCategory,
                "challenge_type": challengeType,
                "challenge_value": challengeValue,
                "dynamic_data": {
                    // Initial value
                    "initial": 500,
                    // Minimum value
                    "minimum": 50,
                    // Amount of solves before challenge reaches minimum value
                    "decay": 10,
                },
            })
        }).then(response => response.json()).then(data => {
            let challenges = document.getElementById("challenges").children;
            for (let challenge of challenges) {
                // Get new data per challenge
                let updateChallenge = data["challenges"][challenge.children["challenge_id"].innerHTML];

                // Update in browser
                challenge.children["category"].innerHTML = updateChallenge["category"];
                challenge.children["type"].innerHTML = updateChallenge["type"];
                challenge.children["value"].innerHTML = updateChallenge["value"];
            }
        });
    });
}
