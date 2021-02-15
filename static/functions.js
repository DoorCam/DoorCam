function hideMessage() {
    //Hide message after 5s
    setTimeout(function() {
        $('.flash').fadeOut("slow", () => {});
    }, 5000);
}