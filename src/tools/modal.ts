export const closemodal = (editProfileModal:HTMLElement | null) => {
   if (editProfileModal){
    editProfileModal.classList.remove('show')
    editProfileModal.setAttribute('aria-hidden', 'true')
    editProfileModal.style.display = 'none'
    editProfileModal.removeAttribute('aria-modal')
    editProfileModal.removeAttribute('role')

    const backdrops = document.querySelectorAll('.modal-backdrop')
    backdrops.forEach((backdrop) => backdrop.remove())

    document.body.classList.remove('modal-open')
    document.body.removeAttribute('data-bs-overflow')
    document.body.removeAttribute('data-bs-padding-right')

    document.body.style.overflow = ''
    document.body.style.paddingRight = ''
   }
}