# tellonym-clone-backend
Api for https://github.com/dariuszzz/tellonym-clone

## Requirements:
- min rust 1.65 nightly
- mysql database named `tellonym_clone` on localhost
- .env with DATABASE_URL and ACCESS_SECRET

# Routes
(dates are iso8601)


- `/register` <br>
  => `json { username: string, password: string }` <br>
  <= access token (plaintext), [ todo: refresh token in cookie ]

- `/login` <br>
  => `json { username: string, password: string }` <br>
  <= access token (plaintext), [ todo: refresh token in cookie ]

- `/user/<id: int>` <br>
  <= `json { id: int, username: string }`

- `/ask` <br>
  => `json { asked_id: int, content: string }`
  
- `/answer` <br>
  => `json { question_id: int, content, string }`

- `/questions/<id: int>` <br>
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
