const isBlankString = (event, value, field) => {
  if (!value.replace(/\s/g, '').length) {
    event.preventDefault();
    alert(`${field} can't be empty`);
  }
};

export default isBlankString;
