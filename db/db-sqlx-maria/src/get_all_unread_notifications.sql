-- gets all unread notifications a user has
SELECT 
    mcaptcha_notifications.id,
    mcaptcha_notifications.heading,
    mcaptcha_notifications.message,
    mcaptcha_notifications.received,
    mcaptcha_users.name
FROM
    mcaptcha_notifications 
INNER JOIN 
    mcaptcha_users 
ON 
    mcaptcha_notifications.tx = mcaptcha_users.id
WHERE 
    mcaptcha_notifications.rx = (
        SELECT 
            id 
        FROM 
            mcaptcha_users
        WHERE
            name = ?
        )
AND 
    mcaptcha_notifications.read_notification IS NULL;
