# Banks Endpoints

## GET `/banks/course/{courseId}`

- Summary: List question banks in a course.
- Auth: Bearer token required.
- Roles (AuthzAction::BankList): `admin`, `func`, `student`.
- Policy: user must have access to the course.
- Path params:
    - `courseId: uuid`
- Response `data`: `QuestionBank[]`.

## GET `/banks/{bankId}`

- Summary: Get one question bank.
- Auth: Bearer token required.
- Roles (AuthzAction::BankRead): `admin`, `func`, `student`.
- Policy: user must have access to the bank/course.
- Path params:
    - `bankId: uuid`
- Response `data`: `QuestionBank`.

## POST `/banks`

- Summary: Create question bank.
- Auth: Bearer token required.
- Roles (AuthzAction::BankCreate): `admin`, `func`, `student`.
- Policy: service enforces course accessibility.
- Body:

```json
{
	"courseId": "uuid",
	"name": "Banco Algebra",
	"questions": [
		{
			"prompt": "2 + 2 = ?",
			"options": ["3", "4"],
			"answerIndex": 1,
			"justification": "Porque $2 + 2 = 4$",
			"images": []
		}
	]
}
```

- Response `data`: none.

## PATCH `/banks/{bankId}`

- Summary: Rename the bank and/or replace its questions.
- Auth: Bearer token required.
- Roles (AuthzAction::BankUpdate): `admin`, `func`, `student`.
- Path params:
    - `bankId: uuid`
- Body (at least one field). `questions` replaces the whole list: each item carries `prompt`, `options` (2–5), `answerIndex` (index into `options`) and an optional `justification`, so the correct answer, the options and the number of questions can all change (1–500 questions). `justification` is Markdown/LaTeX, max 5000 characters; `null` or a blank string stores no justification.

```json
{
	"name": "Banco actualizado",
	"questions": [
		{
			"prompt": "Enunciado editado (soporta Markdown)",
			"options": ["Opción A", "Opción B"],
			"answerIndex": 1,
			"justification": null
		}
	]
}
```

- Response `data`: none.

## DELETE `/banks/{bankId}`

- Summary: Soft-delete bank.
- Auth: Bearer token required.
- Roles (AuthzAction::BankDelete): `admin`, `func`, `student`.
- Path params:
    - `bankId: uuid`
- Response `data`: none.

## Common Domain Errors

- `409`: locked by a running quiz, or the new question count leaves a not-started linked quiz without enough questions.
- `404`: bank not found.
- `400`: invalid question payload.
