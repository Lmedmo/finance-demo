<script lang="ts">
    import { Content } from '$lib/layouts';
    import { CurrentUser } from '$lib/controllers';
    import Header from '$lib/components/header/header.svelte';
    import Setting from '$lib/components/setting/setting.svelte';
    import SettingVal from '$lib/components/setting/settingval.svelte';
    import SettingHeading from '$lib/components/setting/settingheading.svelte';
    import SettingGroup from '$lib/components/setting/group.svelte';
</script>

<Content>
    <Header src={$CurrentUser}/>
    <SettingGroup>
        <SettingHeading value="Profile" />
        <Setting name="Profile Picture">
            <SettingVal value={$CurrentUser.icon} />
        </Setting>
        <Setting name="Username">
            <SettingVal value={$CurrentUser.username} />
        </Setting>
        <Setting name="First Name">
            <SettingVal value={$CurrentUser.first_name} />
        </Setting>
        <Setting name="Last Name">
            <SettingVal value={$CurrentUser.last_name} />
        </Setting>
    </SettingGroup>
</Content>