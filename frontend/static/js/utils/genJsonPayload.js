const genJsonPayload = payload => {
  let value = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  };
  return value;
};

export default genJsonPayload;
