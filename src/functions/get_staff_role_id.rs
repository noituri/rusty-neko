use crate::enums::staff_roles::StaffRoles;
use crate::util::constants::staff_roles::get_staff_roles;

pub fn get_staff_role_id(r: &StaffRoles) -> u64 {
    let roles = get_staff_roles();

    return match r {
        StaffRoles::LeadStaff => *roles.get("Lead Staff").unwrap(),

        StaffRoles::Support => *roles.get("Support").unwrap(),

        StaffRoles::Moderator => *roles.get("Moderator").unwrap(),
    };
}
