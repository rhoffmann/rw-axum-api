window.addEventListener("load", init);

const Session = Object.seal({
  save: function (data) {
    localStorage.setItem("session", JSON.stringify(data));
  },
  load: function () {
    const data = localStorage.getItem("session");
    return data ? JSON.parse(data) : null;
  },
  clear: function () {
    localStorage.removeItem("session");
  },
});

function init() {
  console.log("App initializing...");

  const session = Session.load();

  if (session) {
    console.log("Existing session found:", session);
  }

  const handleLogin = async (e) => {
    e.preventDefault();

    const formData = new FormData(e.target);

    const loginPayload = {
      user: {
        email: formData.get("email"),
        password: formData.get("password"),
      },
    };

    try {
      const response = await fetch("/api/users/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(loginPayload),
      });

      if (response.ok) {
        const result = await response.json();
        Session.save(result);
        alert("Login successful: " + result.message);
      } else {
        const error = await response.json();
        alert("Login failed: " + error.message);
      }
    } catch (error) {
      console.error("Error during login:", error);
      alert("An error occurred. Please try again later.");
    }
  };

  document.querySelector("#loginForm").addEventListener("submit", handleLogin);
}
