# tellonym-clone-backend
Api for https://github.com/dariuszzz/tellonym-clone

## Requirements:
- min rust 1.65 nightly
- mysql database named `tellonym_clone` on localhost
- .env with DATABASE_URL, REFRESH_SECRET and ACCESS_SECRET

# Routes
(dates are iso8601)


- POST `/register` <br>
  => `json { username: string, password: string }` <br>
  <= access token (plaintext), refresh token in cookie

- POST `/login` <br>
  => `json { username: string, password: string }` <br>
  <= access token (plaintext), refresh token in cookie

- GET `/users/<id: int>` <br>
  <= `json { id: int, username: string }`

- POST `/ask` <br>
  => `json { asked_id: int, content: string }`
  Authorization header with access token
  
- POST `/answer` <br>
  => `json { question_id: int, content, string }`
  Authorization header with access token

- GET `/users/<id: int>/questions` <br>
  <=

        json [
            { 
                answer: {
                    answered_at: date,
                    content: string,
                    id: int,
                    last_edit_at: date,
                    likes: int,
                    question_id: int (question which was answered)
                },
                question: {
                    asked_at: date, 
                    asked_id: int, (user who was asked)
                    content: string,
                    id: int,
                    likes: int
                }
            },
            ...
        ] 
    ``

- GET `/users [?search=<string>]` <br>
  <= `json [ user, user2, ...]`
  search= is for filtering by name

- GET `/user` <br>
  <= `json { id: int, username: string }`
  Logged in user
  Authorization header with access token
